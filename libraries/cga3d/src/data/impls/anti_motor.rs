use crate::traits::GeometricProduct;
use crate::traits::RightDual;
use crate::traits::Wedge;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 114
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       4       0
//  Average:        14      19       0
//  Maximum:       224     256       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       6       0
//  Average:        14      19       0
//  Maximum:       224     256       0
impl std::ops::Add<AntiCircleRotor> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (other.group2()[3] + self.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group1()[0] + self.group0()[0]),
                (other.group1()[1] + self.group0()[1]),
                (other.group1()[2] + self.group0()[2]),
                other.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group2()[0] + self.group1()[0]),
                (other.group2()[1] + self.group1()[1]),
                (other.group2()[2] + self.group1()[2]),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversion> for AntiMotor {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]]),
            // e5
            other.group3()[3],
            // e15, e25, e35, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e1234
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiDualNum> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiDualNum) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[1] + self.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], other.group0()[0]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlatPoint> for AntiMotor {
    type Output = MultiVector;
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e1234
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlector> for AntiMotor {
    type Output = MultiVector;
    fn add(self, other: AntiFlector) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e5
            other.group1()[3],
            // e15, e25, e35, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e1234
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiLine> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: AntiLine) -> Self::Output {
        let addition = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiLine> for AntiMotor {
    fn add_assign(&mut self, other: AntiLine) {
        let addition = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiMotor> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: AntiMotor) -> Self::Output {
        let addition = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (other.group0() + self.group0()),
            // e15, e25, e35, e3215
            (other.group1() + self.group1()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiMotor> for AntiMotor {
    fn add_assign(&mut self, other: AntiMotor) {
        let addition = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (other.group0() + self.group0()),
            // e15, e25, e35, e3215
            (other.group1() + self.group1()),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiPlane> for AntiMotor {
    type Output = MultiVector;
    fn add(self, other: AntiPlane) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e5
            other.group0()[3],
            // e15, e25, e35, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e1234
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiScalar> for AntiMotor {
    type Output = MultiVector;
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e1234
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<Circle> for AntiMotor {
    type Output = MultiVector;
    fn add(self, other: Circle) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e1234
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotor> for AntiMotor {
    type Output = MultiVector;
    fn add(self, other: CircleRotor) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e1234
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<Dipole> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: Dipole) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group0()[0] + other.group1()[0]),
                (self.group0()[1] + other.group1()[1]),
                (self.group0()[2] + other.group1()[2]),
                other.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group2()[0] + self.group1()[0]),
                (other.group2()[1] + self.group1()[1]),
                (other.group2()[2] + self.group1()[2]),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversion> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: DipoleInversion) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group0()[0] + other.group1()[0]),
                (self.group0()[1] + other.group1()[1]),
                (self.group0()[2] + other.group1()[2]),
                other.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] + other.group2()[0]),
                (self.group1()[1] + other.group2()[1]),
                (self.group1()[2] + other.group2()[2]),
                other.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], (self.group1()[3] + other.group3()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<DualNum> for AntiMotor {
    type Output = MultiVector;
    fn add(self, other: DualNum) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e1234
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<FlatPoint> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: FlatPoint) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group0()[3]]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<Flector> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: Flector) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group0()[3]]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], (self.group1()[3] + other.group1()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<Line> for AntiMotor {
    type Output = MultiVector;
    fn add(self, other: Line) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e1234
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<Motor> for AntiMotor {
    type Output = MultiVector;
    fn add(self, other: Motor) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other.group1()[3],
            // e15, e25, e35, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e1234
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<MultiVector> for AntiMotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd        8        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[0] + self.group0()[3]), other.group0()[1]]),
            // e1, e2, e3, e4
            other.group1(),
            // e5
            other[e1],
            // e15, e25, e35, e45
            Simd32x4::from([
                (self.group1()[0] + other.group3()[0]),
                (self.group1()[1] + other.group3()[1]),
                (self.group1()[2] + other.group3()[2]),
                other.group3()[3],
            ]),
            // e41, e42, e43
            other.group4(),
            // e23, e31, e12
            (Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) + other.group5()),
            // e415, e425, e435, e321
            other.group6(),
            // e423, e431, e412
            other.group7(),
            // e235, e315, e125
            other.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group9()[0], other.group9()[1], other.group9()[2], (self.group1()[3] + other.group9()[3])]),
            // e1234
            other[e45],
        );
        return addition;
    }
}
impl std::ops::Add<Plane> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Plane) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (self.group1()[3] + other.group0()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<RoundPoint> for AntiMotor {
    type Output = MultiVector;
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            other.group0(),
            // e5
            other[e2],
            // e15, e25, e35, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e1234
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<Scalar> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let addition = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other[scalar])]),
            // e15, e25, e35, e3215
            self.group1(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Scalar> for AntiMotor {
    fn add_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        let addition = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other[scalar])]),
            // e15, e25, e35, e3215
            self.group1(),
        );
        *self = addition;
    }
}
impl std::ops::Add<Sphere> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], other[e4315]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (self.group1()[3] + other.group0()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<VersorEven> for AntiMotor {
    type Output = MultiVector;
    fn add(self, other: VersorEven) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other.group0()[3]]),
            // e1, e2, e3, e4
            other.group3(),
            // e5
            other.group2()[3],
            // e15, e25, e35, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e1234
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<VersorOdd> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: VersorOdd) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (self.group0()[3] + other.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group0()[0] + other.group1()[0]),
                (self.group0()[1] + other.group1()[1]),
                (self.group0()[2] + other.group1()[2]),
                other.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] + other.group2()[0]),
                (self.group1()[1] + other.group2()[1]),
                (self.group1()[2] + other.group2()[2]),
                other.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], (self.group1()[3] + other.group3()[3])]),
        );
        return addition;
    }
}
impl std::ops::BitXor<AntiCircleRotor> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       20       36        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for AntiMotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       40        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDualNum> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        9        0
    fn bitxor(self, other: AntiDualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatPoint> for AntiMotor {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn bitxor(self, other: AntiFlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlector> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12       20        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLine> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiLine> for AntiMotor {
    fn bitxor_assign(&mut self, other: AntiLine) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       21        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMotor> for AntiMotor {
    fn bitxor_assign(&mut self, other: AntiMotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       16        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiScalar> for AntiMotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Circle> for AntiMotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       16        0
    fn bitxor(self, other: Circle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotor> for AntiMotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6       17        0
    fn bitxor(self, other: CircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Dipole> for AntiMotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       28        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversion> for AntiMotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       18       33        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DualNum> for AntiMotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        9        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPoint> for AntiMotor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2       10        0
    fn bitxor(self, other: FlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for AntiMotor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6       14        0
    fn bitxor(self, other: Flector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Line> for AntiMotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        9        0
    fn bitxor(self, other: Line) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for AntiMotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6       14        0
    fn bitxor(self, other: Motor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for AntiMotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       50       82        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Plane> for AntiMotor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn bitxor(self, other: Plane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for AntiMotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       24        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for AntiMotor {
    fn bitxor_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Sphere> for AntiMotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        5        0
    fn bitxor(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for AntiMotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       25       41        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       25       41        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}

impl From<AntiLine> for AntiMotor {
    fn from(anti_line: AntiLine) -> Self {
        use crate::elements::*;
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([anti_line[e23], anti_line[e31], anti_line[e12], 0.0]),
            // e15, e25, e35, e3215
            Simd32x4::from([anti_line[e15], anti_line[e25], anti_line[e35], 0.0]),
        );
    }
}

impl From<Scalar> for AntiMotor {
    fn from(scalar: Scalar) -> Self {
        use crate::elements::*;
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, scalar[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Mul<AntiCircleRotor> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       72       88        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for AntiMotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32      104      120        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4       20        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12       20        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       40       48        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       28       36        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiLine> for AntiMotor {
    fn mul_assign(&mut self, other: AntiLine) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       40       48        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMotor> for AntiMotor {
    fn mul_assign(&mut self, other: AntiMotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       20       28        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for AntiMotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for AntiMotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       64       80        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for AntiMotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       72       88        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       64       80        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32      104      120        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for AntiMotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4       24        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for AntiMotor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12       20        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for AntiMotor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       40       48        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for AntiMotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       28       36        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for AntiMotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       40       48        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for AntiMotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32      224      256        0
    fn mul(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for AntiMotor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       20       28        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for AntiMotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       40        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn mul(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for AntiMotor {
    fn mul_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       40        0
    fn mul(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for AntiMotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32      112      128        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32      112      128        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Neg for AntiMotor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn neg(self) -> Self {
        let negation = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (self.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35, e3215
            (self.group1() * Simd32x4::from(-1.0)),
        );
        return negation;
    }
}
impl std::ops::Not for AntiMotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiCircleRotor> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        4        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (-other.group2()[3] + self.group0()[3]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group1()[0] + self.group0()[0]),
                (-other.group1()[1] + self.group0()[1]),
                (-other.group1()[2] + self.group0()[2]),
                (other.group1()[3] * -1.0),
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group2()[0] + self.group1()[0]),
                (-other.group2()[1] + self.group1()[1]),
                (-other.group2()[2] + self.group1()[2]),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversion> for AntiMotor {
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
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            (Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]]) * Simd32x4::from(-1.0)),
            // e5
            (other.group3()[3] * -1.0),
            // e15, e25, e35, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e1234
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDualNum> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        1        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (-other.group0()[1] + self.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[0] * -1.0)]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlatPoint> for AntiMotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e1234
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlector> for AntiMotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e5
            (other.group1()[3] * -1.0),
            // e15, e25, e35, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e1234
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiLine> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn sub(self, other: AntiLine) -> Self::Output {
        let subtraction = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiLine> for AntiMotor {
    fn sub_assign(&mut self, other: AntiLine) {
        let subtraction = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiMotor> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        let subtraction = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (-other.group0() + self.group0()),
            // e15, e25, e35, e3215
            (-other.group1() + self.group1()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiMotor> for AntiMotor {
    fn sub_assign(&mut self, other: AntiMotor) {
        let subtraction = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (-other.group0() + self.group0()),
            // e15, e25, e35, e3215
            (-other.group1() + self.group1()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiPlane> for AntiMotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e5
            (other.group0()[3] * -1.0),
            // e15, e25, e35, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e1234
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiScalar> for AntiMotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], (other[e12345] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e1234
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<Circle> for AntiMotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn sub(self, other: Circle) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group2() * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e1234
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotor> for AntiMotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       11        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], (other.group2()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e1234
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<Dipole> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        4        0
    fn sub(self, other: Dipole) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group0()[0] - other.group1()[0]),
                (self.group0()[1] - other.group1()[1]),
                (self.group0()[2] - other.group1()[2]),
                (other.group1()[3] * -1.0),
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group2()[0] + self.group1()[0]),
                (-other.group2()[1] + self.group1()[1]),
                (-other.group2()[2] + self.group1()[2]),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversion> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group0()[0] - other.group1()[0]),
                (self.group0()[1] - other.group1()[1]),
                (self.group0()[2] - other.group1()[2]),
                (other.group1()[3] * -1.0),
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] - other.group2()[0]),
                (self.group1()[1] - other.group2()[1]),
                (self.group1()[2] - other.group2()[2]),
                (other.group2()[3] * -1.0),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group3()[0] * -1.0),
                (other.group3()[1] * -1.0),
                (other.group3()[2] * -1.0),
                (self.group1()[3] - other.group3()[3]),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DualNum> for AntiMotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: DualNum) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], (other.group0()[1] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e1234
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlatPoint> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] * -1.0)]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Flector> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        4        0
    fn sub(self, other: Flector) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] * -1.0)]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group1()[0] * -1.0),
                (other.group1()[1] * -1.0),
                (other.group1()[2] * -1.0),
                (self.group1()[3] - other.group1()[3]),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Line> for AntiMotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn sub(self, other: Line) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (other.group1() * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e1234
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<Motor> for AntiMotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn sub(self, other: Motor) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            (other.group1()[3] * -1.0),
            // e15, e25, e35, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e1234
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<MultiVector> for AntiMotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        1        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        8       24        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(-other.group0()[0] + self.group0()[3]), (other.group0()[1] * -1.0)]),
            // e1, e2, e3, e4
            (other.group1() * Simd32x4::from(-1.0)),
            // e5
            (other[e1] * -1.0),
            // e15, e25, e35, e45
            Simd32x4::from([
                (self.group1()[0] - other.group3()[0]),
                (self.group1()[1] - other.group3()[1]),
                (self.group1()[2] - other.group3()[2]),
                (other.group3()[3] * -1.0),
            ]),
            // e41, e42, e43
            (other.group4() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) - other.group5()),
            // e415, e425, e435, e321
            (other.group6() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group7() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group8() * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group9()[0] * -1.0),
                (other.group9()[1] * -1.0),
                (other.group9()[2] * -1.0),
                (self.group1()[3] - other.group9()[3]),
            ]),
            // e1234
            (other[e45] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Plane> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn sub(self, other: Plane) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (self.group1()[3] - other.group0()[3]),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<RoundPoint> for AntiMotor {
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
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            (other.group0() * Simd32x4::from(-1.0)),
            // e5
            (other[e2] * -1.0),
            // e15, e25, e35, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e1234
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<Scalar> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other[scalar])]),
            // e15, e25, e35, e3215
            self.group1(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Scalar> for AntiMotor {
    fn sub_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        let subtraction = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other[scalar])]),
            // e15, e25, e35, e3215
            self.group1(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Sphere> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        4        0
    fn sub(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other[e4315] * -1.0)]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (self.group1()[3] - other.group0()[3]),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEven> for AntiMotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       16        0
    fn sub(self, other: VersorEven) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            (other.group3() * Simd32x4::from(-1.0)),
            // e5
            (other.group2()[3] * -1.0),
            // e15, e25, e35, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e1234
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorOdd> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        8        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (self.group0()[3] - other.group0()[3]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group0()[0] - other.group1()[0]),
                (self.group0()[1] - other.group1()[1]),
                (self.group0()[2] - other.group1()[2]),
                (other.group1()[3] * -1.0),
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] - other.group2()[0]),
                (self.group1()[1] - other.group2()[1]),
                (self.group1()[2] - other.group2()[2]),
                (other.group2()[3] * -1.0),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group3()[0] * -1.0),
                (other.group3()[1] * -1.0),
                (other.group3()[2] * -1.0),
                (self.group1()[3] - other.group3()[3]),
            ]),
        );
        return subtraction;
    }
}

impl TryFrom<AntiCircleRotor> for AntiMotor {
    type Error = String;
    fn try_from(anti_circle_rotor: AntiCircleRotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotor do not fit into AntiMotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([anti_circle_rotor[e23], anti_circle_rotor[e31], anti_circle_rotor[e12], anti_circle_rotor[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from([anti_circle_rotor[e15], anti_circle_rotor[e25], anti_circle_rotor[e35], 0.0]),
        ));
    }
}

impl TryFrom<AntiDualNum> for AntiMotor {
    type Error = String;
    fn try_from(anti_dual_num: AntiDualNum) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dual_num[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDualNum do not fit into AntiMotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, anti_dual_num[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<Dipole> for AntiMotor {
    type Error = String;
    fn try_from(dipole: Dipole) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Dipole do not fit into AntiMotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([dipole[e23], dipole[e31], dipole[e12], 0.0]),
            // e15, e25, e35, e3215
            Simd32x4::from([dipole[e15], dipole[e25], dipole[e35], 0.0]),
        ));
    }
}

impl TryFrom<DipoleInversion> for AntiMotor {
    type Error = String;
    fn try_from(dipole_inversion: DipoleInversion) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversion do not fit into AntiMotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([dipole_inversion[e23], dipole_inversion[e31], dipole_inversion[e12], 0.0]),
            // e15, e25, e35, e3215
            Simd32x4::from([dipole_inversion[e15], dipole_inversion[e25], dipole_inversion[e35], dipole_inversion[e3215]]),
        ));
    }
}

impl TryFrom<FlatPoint> for AntiMotor {
    type Error = String;
    fn try_from(flat_point: FlatPoint) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = flat_point[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from FlatPoint do not fit into AntiMotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
            // e15, e25, e35, e3215
            Simd32x4::from([flat_point[e15], flat_point[e25], flat_point[e35], 0.0]),
        ));
    }
}

impl TryFrom<Flector> for AntiMotor {
    type Error = String;
    fn try_from(flector: Flector) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = flector[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Flector do not fit into AntiMotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
            // e15, e25, e35, e3215
            Simd32x4::from([flector[e15], flector[e25], flector[e35], flector[e3215]]),
        ));
    }
}

impl TryFrom<MultiVector> for AntiMotor {
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
        let el = multi_vector[31];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MultiVector do not fit into AntiMotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([multi_vector[e23], multi_vector[e31], multi_vector[e12], multi_vector[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from([multi_vector[e15], multi_vector[e25], multi_vector[e35], multi_vector[e3215]]),
        ));
    }
}

impl TryFrom<Plane> for AntiMotor {
    type Error = String;
    fn try_from(plane: Plane) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = plane[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = plane[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = plane[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Plane do not fit into AntiMotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
            // e15, e25, e35, e3215
            Simd32x4::from([0.0, 0.0, 0.0, plane[e3215]]),
        ));
    }
}

impl TryFrom<Sphere> for AntiMotor {
    type Error = String;
    fn try_from(sphere: Sphere) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = sphere[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = sphere[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = sphere[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = sphere[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Sphere do not fit into AntiMotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
            // e15, e25, e35, e3215
            Simd32x4::from([0.0, 0.0, 0.0, sphere[e3215]]),
        ));
    }
}

impl TryFrom<VersorOdd> for AntiMotor {
    type Error = String;
    fn try_from(versor_odd: VersorOdd) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[14];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOdd do not fit into AntiMotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([versor_odd[e23], versor_odd[e31], versor_odd[e12], versor_odd[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from([versor_odd[e15], versor_odd[e25], versor_odd[e35], versor_odd[e3215]]),
        ));
    }
}
