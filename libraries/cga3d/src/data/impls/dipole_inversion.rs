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
// Total Implementations: 110
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         5       4       0
//  Average:        29      34       0
//  Maximum:       448     480       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         6       6       0
//  Average:        30      35       0
//  Maximum:       448     480       0
impl std::ops::Add<AntiCircleRotor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        7        0        0
    //  no simd       10        0        0
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                other.group2()[3],
            ]),
            // e23, e31, e12, e45
            (other.group1() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group2()[0] + self.group2()[0]),
                (other.group2()[1] + self.group2()[1]),
                (other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversion> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]]),
            // e5
            other.group3()[3],
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiDualNum> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiDualNum) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group0()[1]]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[0] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlatPoint> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlector> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiFlector) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e5
            other.group1()[3],
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiLine> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: AntiLine) -> Self::Output {
        let addition = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiLine> for DipoleInversion {
    fn add_assign(&mut self, other: AntiLine) {
        let addition = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiMotor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: AntiMotor) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group1()[3] + self.group3()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiPlane> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiPlane) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e5
            other.group0()[3],
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiScalar> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<Circle> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: Circle) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotor> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: CircleRotor) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<Dipole> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       10        0        0
    fn add(self, other: Dipole) -> Self::Output {
        let addition = DipoleInversion::from_groups(
            // e41, e42, e43
            (other.group0() + self.group0()),
            // e23, e31, e12, e45
            (other.group1() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group2()[0] + self.group2()[0]),
                (other.group2()[1] + self.group2()[1]),
                (other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Dipole> for DipoleInversion {
    fn add_assign(&mut self, other: Dipole) {
        let addition = DipoleInversion::from_groups(
            // e41, e42, e43
            (other.group0() + self.group0()),
            // e23, e31, e12, e45
            (other.group1() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group2()[0] + self.group2()[0]),
                (other.group2()[1] + self.group2()[1]),
                (other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<DipoleInversion> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       15        0        0
    fn add(self, other: DipoleInversion) -> Self::Output {
        let addition = DipoleInversion::from_groups(
            // e41, e42, e43
            (other.group0() + self.group0()),
            // e23, e31, e12, e45
            (other.group1() + self.group1()),
            // e15, e25, e35, e1234
            (other.group2() + self.group2()),
            // e4235, e4315, e4125, e3215
            (other.group3() + self.group3()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<DipoleInversion> for DipoleInversion {
    fn add_assign(&mut self, other: DipoleInversion) {
        let addition = DipoleInversion::from_groups(
            // e41, e42, e43
            (other.group0() + self.group0()),
            // e23, e31, e12, e45
            (other.group1() + self.group1()),
            // e15, e25, e35, e1234
            (other.group2() + self.group2()),
            // e4235, e4315, e4125, e3215
            (other.group3() + self.group3()),
        );
        *self = addition;
    }
}
impl std::ops::Add<DualNum> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: DualNum) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<FlatPoint> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: FlatPoint) -> Self::Output {
        let addition = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group2()[0] + other.group0()[0]),
                (self.group2()[1] + other.group0()[1]),
                (self.group2()[2] + other.group0()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<FlatPoint> for DipoleInversion {
    fn add_assign(&mut self, other: FlatPoint) {
        let addition = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group2()[0] + other.group0()[0]),
                (self.group2()[1] + other.group0()[1]),
                (self.group2()[2] + other.group0()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<Flector> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: Flector) -> Self::Output {
        let addition = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group2()[0] + other.group0()[0]),
                (self.group2()[1] + other.group0()[1]),
                (self.group2()[2] + other.group0()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            (self.group3() + other.group1()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Flector> for DipoleInversion {
    fn add_assign(&mut self, other: Flector) {
        let addition = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group2()[0] + other.group0()[0]),
                (self.group2()[1] + other.group0()[1]),
                (self.group2()[2] + other.group0()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            (self.group3() + other.group1()),
        );
        *self = addition;
    }
}
impl std::ops::Add<Line> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: Line) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<Motor> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: Motor) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other.group1()[3],
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<MultiVector> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       15        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            other.group0(),
            // e1, e2, e3, e4
            other.group1(),
            // e5
            other[e1],
            // e15, e25, e35, e45
            (Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]) + other.group3()),
            // e41, e42, e43
            (self.group0() + other.group4()),
            // e23, e31, e12
            (Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) + other.group5()),
            // e415, e425, e435, e321
            other.group6(),
            // e423, e431, e412
            other.group7(),
            // e235, e315, e125
            other.group8(),
            // e4235, e4315, e4125, e3215
            (self.group3() + other.group9()),
            // e1234
            (self.group2()[3] + other[e45]),
        );
        return addition;
    }
}
impl std::ops::Add<Plane> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: Plane) -> Self::Output {
        let addition = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            (self.group3() + other.group0()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Plane> for DipoleInversion {
    fn add_assign(&mut self, other: Plane) {
        let addition = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            (self.group3() + other.group0()),
        );
        *self = addition;
    }
}
impl std::ops::Add<RoundPoint> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group0(),
            // e5
            other[e2],
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<Scalar> for DipoleInversion {
    type Output = VersorOdd;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other[scalar]]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::Add<Sphere> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let addition = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other[e4315])]),
            // e4235, e4315, e4125, e3215
            (self.group3() + other.group0()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Sphere> for DipoleInversion {
    fn add_assign(&mut self, other: Sphere) {
        use crate::elements::*;
        let addition = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other[e4315])]),
            // e4235, e4315, e4125, e3215
            (self.group3() + other.group0()),
        );
        *self = addition;
    }
}
impl std::ops::Add<VersorEven> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: VersorEven) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other.group0()[3]]),
            // e1, e2, e3, e4
            other.group3(),
            // e5
            other.group2()[3],
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorOdd> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       15        0        0
    fn add(self, other: VersorOdd) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                other.group0()[3],
            ]),
            // e23, e31, e12, e45
            (self.group1() + other.group1()),
            // e15, e25, e35, e1234
            (self.group2() + other.group2()),
            // e4235, e4315, e4125, e3215
            (self.group3() + other.group3()),
        );
        return addition;
    }
}
impl std::ops::BitXor<AntiCircleRotor> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       30       45        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotor> for DipoleInversion {
    fn bitxor_assign(&mut self, other: AntiCircleRotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for DipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       34       45        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDualNum> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       15        0
    fn bitxor(self, other: AntiDualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiDualNum> for DipoleInversion {
    fn bitxor_assign(&mut self, other: AntiDualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatPoint> for DipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: AntiFlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlector> for DipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       21       32        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLine> for DipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       18        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       18       33        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMotor> for DipoleInversion {
    fn bitxor_assign(&mut self, other: AntiMotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for DipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       17       28        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Circle> for DipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn bitxor(self, other: Circle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotor> for DipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn bitxor(self, other: CircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Dipole> for DipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       25       30        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversion> for DipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       25       30        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DualNum> for DipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        7        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPoint> for DipoleInversion {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn bitxor(self, other: FlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for DipoleInversion {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn bitxor(self, other: Flector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Line> for DipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: Line) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for DipoleInversion {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6       13        0
    fn bitxor(self, other: Motor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       64       90        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for DipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       35        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       15        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for DipoleInversion {
    fn bitxor_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for DipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       34       45        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       30       45        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOdd> for DipoleInversion {
    fn bitxor_assign(&mut self, other: VersorOdd) {
        *self = self.wedge(other);
    }
}

impl From<AntiLine> for DipoleInversion {
    fn from(anti_line: AntiLine) -> Self {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([anti_line[e23], anti_line[e31], anti_line[e12], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([anti_line[e15], anti_line[e25], anti_line[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<Dipole> for DipoleInversion {
    fn from(dipole: Dipole) -> Self {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from([dipole[e41], dipole[e42], dipole[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([dipole[e23], dipole[e31], dipole[e12], dipole[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([dipole[e15], dipole[e25], dipole[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<FlatPoint> for DipoleInversion {
    fn from(flat_point: FlatPoint) -> Self {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, flat_point[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([flat_point[e15], flat_point[e25], flat_point[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<Flector> for DipoleInversion {
    fn from(flector: Flector) -> Self {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, flector[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([flector[e15], flector[e25], flector[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([flector[e4235], flector[e4315], flector[e4125], flector[e3215]]),
        );
    }
}

impl From<Plane> for DipoleInversion {
    fn from(plane: Plane) -> Self {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([plane[e4235], plane[e4315], plane[e4125], plane[e3215]]),
        );
    }
}

impl From<Sphere> for DipoleInversion {
    fn from(sphere: Sphere) -> Self {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, sphere[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([sphere[e4235], sphere[e4315], sphere[e4125], sphere[e3215]]),
        );
    }
}
impl std::ops::Mul<AntiCircleRotor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32      149      165        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32      209      225        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       30        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       44       64        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32      104      120        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       74       90        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32      104      120        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       44       60        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       19        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32      134      150        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32      149      165        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32      134      150        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32      209      225        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       30        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       44       60        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32      104      120        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       74       90        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32      104      120        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32      448      480        0
    fn mul(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       44       60        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       59       75        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       15        0
    fn mul(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for DipoleInversion {
    fn mul_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       59       75        0
    fn mul(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32      224      240        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32      224      240        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Neg for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn neg(self) -> Self {
        let negation = DipoleInversion::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(-1.0)),
            // e15, e25, e35, e1234
            (self.group2() * Simd32x4::from(-1.0)),
            // e4235, e4315, e4125, e3215
            (self.group3() * Simd32x4::from(-1.0)),
        );
        return negation;
    }
}
impl std::ops::Not for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       12        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiCircleRotor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        7        1        0
    //  no simd       10        1        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                (other.group2()[3] * -1.0),
            ]),
            // e23, e31, e12, e45
            (-other.group1() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group2()[0] + self.group2()[0]),
                (-other.group2()[1] + self.group2()[1]),
                (-other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversion> for DipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]]) * Simd32x4::from(-1.0)),
            // e5
            (other.group3()[3] * -1.0),
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDualNum> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        1        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[1] * -1.0)]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[0] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlatPoint> for DipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlector> for DipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e5
            (other.group1()[3] * -1.0),
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiLine> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn sub(self, other: AntiLine) -> Self::Output {
        let subtraction = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiLine> for DipoleInversion {
    fn sub_assign(&mut self, other: AntiLine) {
        let subtraction = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiMotor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        1        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] * -1.0)]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group1()[3] + self.group3()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiPlane> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e5
            (other.group0()[3] * -1.0),
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiScalar> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other[e12345] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<Circle> for DipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group2() * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotor> for DipoleInversion {
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
            Simd32x2::from([0.0, (other.group2()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<Dipole> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       10        0        0
    fn sub(self, other: Dipole) -> Self::Output {
        let subtraction = DipoleInversion::from_groups(
            // e41, e42, e43
            (-other.group0() + self.group0()),
            // e23, e31, e12, e45
            (-other.group1() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group2()[0] + self.group2()[0]),
                (-other.group2()[1] + self.group2()[1]),
                (-other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Dipole> for DipoleInversion {
    fn sub_assign(&mut self, other: Dipole) {
        let subtraction = DipoleInversion::from_groups(
            // e41, e42, e43
            (-other.group0() + self.group0()),
            // e23, e31, e12, e45
            (-other.group1() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group2()[0] + self.group2()[0]),
                (-other.group2()[1] + self.group2()[1]),
                (-other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<DipoleInversion> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       15        0        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        let subtraction = DipoleInversion::from_groups(
            // e41, e42, e43
            (-other.group0() + self.group0()),
            // e23, e31, e12, e45
            (-other.group1() + self.group1()),
            // e15, e25, e35, e1234
            (-other.group2() + self.group2()),
            // e4235, e4315, e4125, e3215
            (-other.group3() + self.group3()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<DipoleInversion> for DipoleInversion {
    fn sub_assign(&mut self, other: DipoleInversion) {
        let subtraction = DipoleInversion::from_groups(
            // e41, e42, e43
            (-other.group0() + self.group0()),
            // e23, e31, e12, e45
            (-other.group1() + self.group1()),
            // e15, e25, e35, e1234
            (-other.group2() + self.group2()),
            // e4235, e4315, e4125, e3215
            (-other.group3() + self.group3()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<DualNum> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: DualNum) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other.group0()[1] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlatPoint> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        let subtraction = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group2()[0] - other.group0()[0]),
                (self.group2()[1] - other.group0()[1]),
                (self.group2()[2] - other.group0()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<FlatPoint> for DipoleInversion {
    fn sub_assign(&mut self, other: FlatPoint) {
        let subtraction = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group2()[0] - other.group0()[0]),
                (self.group2()[1] - other.group0()[1]),
                (self.group2()[2] - other.group0()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Flector> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn sub(self, other: Flector) -> Self::Output {
        let subtraction = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group2()[0] - other.group0()[0]),
                (self.group2()[1] - other.group0()[1]),
                (self.group2()[2] - other.group0()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            (self.group3() - other.group1()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Flector> for DipoleInversion {
    fn sub_assign(&mut self, other: Flector) {
        let subtraction = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group2()[0] - other.group0()[0]),
                (self.group2()[1] - other.group0()[1]),
                (self.group2()[2] - other.group0()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            (self.group3() - other.group1()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Line> for DipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (other.group1() * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<Motor> for DipoleInversion {
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
            Simd32x2::from([0.0, (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            (other.group1()[3] * -1.0),
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<MultiVector> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        1        0
    //    simd2        0        1        0
    //    simd3        2        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        5        6        0
    //  no simd       15       17        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            (other.group0() * Simd32x2::from(-1.0)),
            // e1, e2, e3, e4
            (other.group1() * Simd32x4::from(-1.0)),
            // e5
            (other[e1] * -1.0),
            // e15, e25, e35, e45
            (Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]) - other.group3()),
            // e41, e42, e43
            (self.group0() - other.group4()),
            // e23, e31, e12
            (Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) - other.group5()),
            // e415, e425, e435, e321
            (other.group6() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group7() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group8() * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            (self.group3() - other.group9()),
            // e1234
            (self.group2()[3] - other[e45]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Plane> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: Plane) -> Self::Output {
        let subtraction = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            (self.group3() - other.group0()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Plane> for DipoleInversion {
    fn sub_assign(&mut self, other: Plane) {
        let subtraction = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            (self.group3() - other.group0()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<RoundPoint> for DipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (other.group0() * Simd32x4::from(-1.0)),
            // e5
            (other[e2] * -1.0),
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<Scalar> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other[scalar] * -1.0)]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Sphere> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn sub(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let subtraction = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other[e4315])]),
            // e4235, e4315, e4125, e3215
            (self.group3() - other.group0()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Sphere> for DipoleInversion {
    fn sub_assign(&mut self, other: Sphere) {
        use crate::elements::*;
        let subtraction = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other[e4315])]),
            // e4235, e4315, e4125, e3215
            (self.group3() - other.group0()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<VersorEven> for DipoleInversion {
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
            Simd32x2::from([0.0, (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            (other.group3() * Simd32x4::from(-1.0)),
            // e5
            (other.group2()[3] * -1.0),
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorOdd> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        6        1        0
    //  no simd       15        1        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                (other.group0()[3] * -1.0),
            ]),
            // e23, e31, e12, e45
            (self.group1() - other.group1()),
            // e15, e25, e35, e1234
            (self.group2() - other.group2()),
            // e4235, e4315, e4125, e3215
            (self.group3() - other.group3()),
        );
        return subtraction;
    }
}

impl TryFrom<AntiCircleRotor> for DipoleInversion {
    type Error = String;
    fn try_from(anti_circle_rotor: AntiCircleRotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotor do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from([anti_circle_rotor[e41], anti_circle_rotor[e42], anti_circle_rotor[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([anti_circle_rotor[e23], anti_circle_rotor[e31], anti_circle_rotor[e12], anti_circle_rotor[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([anti_circle_rotor[e15], anti_circle_rotor[e25], anti_circle_rotor[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiDualNum> for DipoleInversion {
    type Error = String;
    fn try_from(anti_dual_num: AntiDualNum) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dual_num[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDualNum do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, anti_dual_num[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiMotor> for DipoleInversion {
    type Error = String;
    fn try_from(anti_motor: AntiMotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_motor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMotor do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([anti_motor[e23], anti_motor[e31], anti_motor[e12], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([anti_motor[e15], anti_motor[e25], anti_motor[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, anti_motor[e3215]]),
        ));
    }
}

impl TryFrom<MultiVector> for DipoleInversion {
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
            let mut error = "Elements from MultiVector do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from([multi_vector[e41], multi_vector[e42], multi_vector[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([multi_vector[e23], multi_vector[e31], multi_vector[e12], multi_vector[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([multi_vector[e15], multi_vector[e25], multi_vector[e35], multi_vector[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([multi_vector[e4235], multi_vector[e4315], multi_vector[e4125], multi_vector[e3215]]),
        ));
    }
}

impl TryFrom<VersorOdd> for DipoleInversion {
    type Error = String;
    fn try_from(versor_odd: VersorOdd) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOdd do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from([versor_odd[e41], versor_odd[e42], versor_odd[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([versor_odd[e23], versor_odd[e31], versor_odd[e12], versor_odd[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([versor_odd[e15], versor_odd[e25], versor_odd[e35], versor_odd[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([versor_odd[e4235], versor_odd[e4315], versor_odd[e4125], versor_odd[e3215]]),
        ));
    }
}
