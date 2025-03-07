use crate::traits::GeometricProduct;
use crate::traits::RightDual;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 89
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       2       0
//  Average:        15      21       0
//  Maximum:       189     226       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         4       3       0
//  Average:        36      40       0
//  Maximum:       480     512       0
impl std::ops::Add<AntiCircleRotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       11        0        0
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() + other.group0().with_w(other[scalar]),
            // e23, e31, e12, e45
            other.group1() + self.group1(),
            // e15, e25, e35, e1234
            (other.group2().xyz() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
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
            (other.group2().xyz() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<AntiDipoleInversion> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
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
        );
    }
}
impl std::ops::Add<AntiDualNum> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
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
        return MultiVector::from_groups(
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
        );
    }
}
impl std::ops::Add<AntiFlector> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
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
        );
    }
}
impl std::ops::Add<AntiLine> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            (other.group0() + self.group1().xyz()).with_w(self[e45]),
            // e15, e25, e35, e1234
            (other.group1() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<AntiLine> for VersorOdd {
    fn add_assign(&mut self, other: AntiLine) {
        use crate::elements::*;
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            (other.group0() + self.group1().xyz()).with_w(self[e45]),
            // e15, e25, e35, e1234
            (other.group1() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<AntiMotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        8        0        0
    fn add(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().xyz().with_w(other[scalar] + self[scalar]),
            // e23, e31, e12, e45
            (other.group0().xyz() + self.group1().xyz()).with_w(self[e45]),
            // e15, e25, e35, e1234
            (other.group1().xyz() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3().xyz().with_w(other[e3215] + self[e3215]),
        );
    }
}
impl std::ops::AddAssign<AntiMotor> for VersorOdd {
    fn add_assign(&mut self, other: AntiMotor) {
        use crate::elements::*;
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().xyz().with_w(other[scalar] + self[scalar]),
            // e23, e31, e12, e45
            (other.group0().xyz() + self.group1().xyz()).with_w(self[e45]),
            // e15, e25, e35, e1234
            (other.group1().xyz() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3().xyz().with_w(other[e3215] + self[e3215]),
        );
    }
}
impl std::ops::Add<AntiPlane> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
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
        );
    }
}
impl std::ops::Add<AntiScalar> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
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
        );
    }
}
impl std::ops::Add<Circle> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
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
        );
    }
}
impl std::ops::Add<CircleRotor> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
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
        );
    }
}
impl std::ops::Add<Dipole> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn add(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (other.group0() + self.group0().xyz()).with_w(self[scalar]),
            // e23, e31, e12, e45
            other.group1() + self.group1(),
            // e15, e25, e35, e1234
            (other.group2() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<Dipole> for VersorOdd {
    fn add_assign(&mut self, other: Dipole) {
        use crate::elements::*;
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (other.group0() + self.group0().xyz()).with_w(self[scalar]),
            // e23, e31, e12, e45
            other.group1() + self.group1(),
            // e15, e25, e35, e1234
            (other.group2() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<DipoleInversion> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       15        0        0
    fn add(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (other.group0() + self.group0().xyz()).with_w(self[scalar]),
            // e23, e31, e12, e45
            other.group1() + self.group1(),
            // e15, e25, e35, e1234
            other.group2() + self.group2(),
            // e4235, e4315, e4125, e3215
            other.group3() + self.group3(),
        );
    }
}
impl std::ops::AddAssign<DipoleInversion> for VersorOdd {
    fn add_assign(&mut self, other: DipoleInversion) {
        use crate::elements::*;
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (other.group0() + self.group0().xyz()).with_w(self[scalar]),
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
        return MultiVector::from_groups(
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
        );
    }
}
impl std::ops::Add<FlatPoint> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(other[e45] + self[e45]),
            // e15, e25, e35, e1234
            (other.group0().xyz() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
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
            (other.group0().xyz() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<Flector> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        8        0        0
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(other[e45] + self[e45]),
            // e15, e25, e35, e1234
            (other.group0().xyz() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            other.group1() + self.group3(),
        );
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
            (other.group0().xyz() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            other.group1() + self.group3(),
        );
    }
}
impl std::ops::Add<Line> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
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
        );
    }
}
impl std::ops::Add<Motor> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
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
        );
    }
}
impl std::ops::Add<MultiVector> for VersorOdd {
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
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]) + other.group0(),
            // e1, e2, e3, e4
            other.group1(),
            // e5
            other[e5],
            // e15, e25, e35, e45
            other.group3() + self.group2().xyz().with_w(self[e45]),
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
        );
    }
}
impl std::ops::Add<Plane> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: Plane) -> Self::Output {
        return VersorOdd::from_groups(
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
        return MultiVector::from_groups(
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
        );
    }
}
impl std::ops::Add<Scalar> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() + Simd32x3::from(0.0).with_w(other[scalar]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<Scalar> for VersorOdd {
    fn add_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() + Simd32x3::from(0.0).with_w(other[scalar]),
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
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234]),
            // e4235, e4315, e4125, e3215
            other.group0() + self.group3(),
        );
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
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234]),
            // e4235, e4315, e4125, e3215
            other.group0() + self.group3(),
        );
    }
}
impl std::ops::Add<VersorEven> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
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
        );
    }
}
impl std::ops::Add<VersorOdd> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        4        0        0
    // no simd       16        0        0
    fn add(self, other: VersorOdd) -> Self::Output {
        return VersorOdd::from_groups(
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

impl From<AntiCircleRotor> for VersorOdd {
    fn from(from_anti_circle_rotor: AntiCircleRotor) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            from_anti_circle_rotor.group0().with_w(from_anti_circle_rotor[scalar]),
            // e23, e31, e12, e45
            from_anti_circle_rotor.group1(),
            // e15, e25, e35, e1234
            from_anti_circle_rotor.group2().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiDualNum> for VersorOdd {
    fn from(from_anti_dual_num: AntiDualNum) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(from_anti_dual_num[scalar]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(from_anti_dual_num[e3215]),
        );
    }
}

impl From<AntiLine> for VersorOdd {
    fn from(from_anti_line: AntiLine) -> Self {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            from_anti_line.group0().with_w(0.0),
            // e15, e25, e35, e1234
            from_anti_line.group1().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiMotor> for VersorOdd {
    fn from(from_anti_motor: AntiMotor) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(from_anti_motor[scalar]),
            // e23, e31, e12, e45
            from_anti_motor.group0().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            from_anti_motor.group1().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(from_anti_motor[e3215]),
        );
    }
}

impl From<Dipole> for VersorOdd {
    fn from(from_dipole: Dipole) -> Self {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            from_dipole.group0().with_w(0.0),
            // e23, e31, e12, e45
            from_dipole.group1(),
            // e15, e25, e35, e1234
            from_dipole.group2().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<DipoleInversion> for VersorOdd {
    fn from(from_dipole_inversion: DipoleInversion) -> Self {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            from_dipole_inversion.group0().with_w(0.0),
            // e23, e31, e12, e45
            from_dipole_inversion.group1(),
            // e15, e25, e35, e1234
            from_dipole_inversion.group2(),
            // e4235, e4315, e4125, e3215
            from_dipole_inversion.group3(),
        );
    }
}

impl From<FlatPoint> for VersorOdd {
    fn from(from_flat_point: FlatPoint) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_flat_point[e45]),
            // e15, e25, e35, e1234
            from_flat_point.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<Flector> for VersorOdd {
    fn from(from_flector: Flector) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_flector[e45]),
            // e15, e25, e35, e1234
            from_flector.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            from_flector.group1(),
        );
    }
}

impl From<Plane> for VersorOdd {
    fn from(from_plane: Plane) -> Self {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            from_plane.group0(),
        );
    }
}

impl From<Scalar> for VersorOdd {
    fn from(from_scalar: Scalar) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(from_scalar[scalar]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<Sphere> for VersorOdd {
    fn from(from_sphere: Sphere) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_sphere[e1234]),
            // e4235, e4315, e4125, e3215
            from_sphere.group0(),
        );
    }
}
impl std::ops::Mul<AntiCircleRotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       70        0
    //    simd3        0        6        0
    //    simd4       28       22        0
    // Totals...
    // yes simd       76       98        0
    //  no simd      160      176        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
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
    //           add/sub      mul      div
    //      f32       32       63        0
    //    simd3        0       13        0
    //    simd4       48       35        0
    // Totals...
    // yes simd       80      111        0
    //  no simd      224      242        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       16        0
    //    simd3        2        3        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       22        0
    //  no simd       16       37        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
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
    //           add/sub      mul      div
    //      f32       15       27        0
    //    simd3        3        6        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       24       39        0
    //  no simd       48       69        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       50        0
    //    simd3        0        2        0
    //    simd4       20       18        0
    // Totals...
    // yes simd       56       70        0
    //  no simd      116      128        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       56        0
    //    simd3        0        7        0
    //    simd4       12        5        0
    // Totals...
    // yes simd       45       68        0
    //  no simd       81       97        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
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
    //           add/sub      mul      div
    //      f32       33       53        0
    //    simd3        0        5        0
    //    simd4       20       15        0
    // Totals...
    // yes simd       53       73        0
    //  no simd      113      128        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
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
    //           add/sub      mul      div
    //      f32        9       28        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       19       38        0
    //  no simd       49       68        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        6        0
    // no simd        0       24        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       64        0
    //    simd3        0        5        0
    //    simd4       26       21        0
    // Totals...
    // yes simd       66       90        0
    //  no simd      144      163        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       58        0
    //    simd3        0        9        0
    //    simd4       31       23        0
    // Totals...
    // yes simd       67       90        0
    //  no simd      160      177        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       69        0
    //    simd3        0        5        0
    //    simd4       24       19        0
    // Totals...
    // yes simd       72       93        0
    //  no simd      144      160        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
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
    //           add/sub      mul      div
    //      f32       64       88        0
    //    simd3        0        8        0
    //    simd4       40       32        0
    // Totals...
    // yes simd      104      128        0
    //  no simd      224      240        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
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
    //           add/sub      mul      div
    //      f32        1        8        0
    //    simd3        1        2        0
    //    simd4        3        5        0
    // Totals...
    // yes simd        5       15        0
    //  no simd       16       34        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       23        0
    //    simd3        6        8        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       21       36        0
    //  no simd       48       67        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
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
    //           add/sub      mul      div
    //      f32       32       51        0
    //    simd3        0        7        0
    //    simd4       21       14        0
    // Totals...
    // yes simd       53       72        0
    //  no simd      116      128        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
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
    //           add/sub      mul      div
    //      f32       33       53        0
    //    simd3        0        4        0
    //    simd4       12        8        0
    // Totals...
    // yes simd       45       65        0
    //  no simd       81       97        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       37       55        0
    //    simd3        0        3        0
    //    simd4       19       16        0
    // Totals...
    // yes simd       56       74        0
    //  no simd      113      128        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       64       96        0
    //    simd2       12       12        0
    //    simd3       60       80        0
    //    simd4       53       38        0
    // Totals...
    // yes simd      189      226        0
    //  no simd      480      512        0
    fn mul(self, other: MultiVector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       37        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       27       44        0
    //  no simd       48       65        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
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
    //           add/sub      mul      div
    //      f32       16       37        0
    //    simd3        0        2        0
    //    simd4       12       10        0
    // Totals...
    // yes simd       28       49        0
    //  no simd       64       83        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn mul(self, other: Scalar) -> Self::Output {
        return self.geometric_product(other);
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
    //           add/sub      mul      div
    //      f32       16       36        0
    //    simd3        0        3        0
    //    simd4       12        9        0
    // Totals...
    // yes simd       28       48        0
    //  no simd       64       81        0
    fn mul(self, other: Sphere) -> Self::Output {
        return self.geometric_product(other);
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
    //           add/sub      mul      div
    //      f32       36       58        0
    //    simd3        0       10        0
    //    simd4       51       42        0
    // Totals...
    // yes simd       87      110        0
    //  no simd      240      256        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       64       91        0
    //    simd3        0       11        0
    //    simd4       44       33        0
    // Totals...
    // yes simd      108      135        0
    //  no simd      240      256        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
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
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn neg(self) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group3() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Not for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiCircleRotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        3        3        0
    //  no simd       12        3        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() - other.group0().with_w(other[scalar]),
            // e23, e31, e12, e45
            self.group1() - other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<AntiCircleRotor> for VersorOdd {
    fn sub_assign(&mut self, other: AntiCircleRotor) {
        use crate::elements::*;
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() - other.group0().with_w(other[scalar]),
            // e23, e31, e12, e45
            self.group1() - other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversion> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       15        0
    fn sub(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(other[e4]) * Simd32x4::from(-1.0),
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
        );
    }
}
impl std::ops::Sub<AntiDualNum> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        8        2        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() + Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
        );
    }
}
impl std::ops::SubAssign<AntiDualNum> for VersorOdd {
    fn sub_assign(&mut self, other: AntiDualNum) {
        use crate::elements::*;
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() + Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
        );
    }
}
impl std::ops::Sub<AntiFlatPoint> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
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
            Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<AntiFlector> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       11        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
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
            Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<AntiLine> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        8        6        0
    fn sub(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<AntiLine> for VersorOdd {
    fn sub_assign(&mut self, other: AntiLine) {
        use crate::elements::*;
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<AntiMotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        8        0
    //    simd4        4        0        0
    // Totals...
    // yes simd        4        8        0
    //  no simd       16        8        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() + Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
        );
    }
}
impl std::ops::SubAssign<AntiMotor> for VersorOdd {
    fn sub_assign(&mut self, other: AntiMotor) {
        use crate::elements::*;
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() + Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
        );
    }
}
impl std::ops::Sub<AntiPlane> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
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
        );
    }
}
impl std::ops::Sub<AntiScalar> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
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
        );
    }
}
impl std::ops::Sub<Circle> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn sub(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
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
        );
    }
}
impl std::ops::Sub<CircleRotor> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
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
        );
    }
}
impl std::ops::Sub<Dipole> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        3        6        0
    //  no simd       12        6        0
    fn sub(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0(),
            // e23, e31, e12, e45
            self.group1() - other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<Dipole> for VersorOdd {
    fn sub_assign(&mut self, other: Dipole) {
        use crate::elements::*;
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0(),
            // e23, e31, e12, e45
            self.group1() - other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<DipoleInversion> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        4        0        0
    // Totals...
    // yes simd        4        3        0
    //  no simd       16        3        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0(),
            // e23, e31, e12, e45
            self.group1() - other.group1(),
            // e15, e25, e35, e1234
            self.group2() - other.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() - other.group3(),
        );
    }
}
impl std::ops::SubAssign<DipoleInversion> for VersorOdd {
    fn sub_assign(&mut self, other: DipoleInversion) {
        use crate::elements::*;
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0(),
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
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn sub(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
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
        );
    }
}
impl std::ops::Sub<FlatPoint> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        4        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<FlatPoint> for VersorOdd {
    fn sub_assign(&mut self, other: FlatPoint) {
        use crate::elements::*;
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<Flector> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        3        4        0
    //  no simd       12        4        0
    fn sub(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() - other.group1(),
        );
    }
}
impl std::ops::SubAssign<Flector> for VersorOdd {
    fn sub_assign(&mut self, other: Flector) {
        use crate::elements::*;
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() - other.group1(),
        );
    }
}
impl std::ops::Sub<Line> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
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
        );
    }
}
impl std::ops::Sub<Motor> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        9        0
    fn sub(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
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
        );
    }
}
impl std::ops::Sub<MultiVector> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        1        0
    //    simd2        0        1        0
    //    simd3        2        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        6        6        0
    //  no simd       16       17        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
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
        );
    }
}
impl std::ops::Sub<Plane> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: Plane) -> Self::Output {
        return VersorOdd::from_groups(
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
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
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
        );
    }
}
impl std::ops::Sub<Scalar> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<Scalar> for VersorOdd {
    fn sub_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
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
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        8        1        0
    fn sub(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            self.group3() - other.group0(),
        );
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
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            self.group3() - other.group0(),
        );
    }
}
impl std::ops::Sub<VersorEven> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       17        0
    fn sub(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
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
        );
    }
}
impl std::ops::Sub<VersorOdd> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        4        0        0
    // no simd       16        0        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        return VersorOdd::from_groups(
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
        return Ok(VersorOdd::from_groups(
            // e41, e42, e43, scalar
            multi_vector.group4().with_w(multi_vector[scalar]),
            // e23, e31, e12, e45
            multi_vector.group5().with_w(multi_vector[e45]),
            // e15, e25, e35, e1234
            Simd32x4::from([multi_vector[e15], multi_vector[e25], multi_vector[e35], multi_vector[e1234]]),
            // e4235, e4315, e4125, e3215
            multi_vector.group9(),
        ));
    }
}
