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
// Total Implementations: 107
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         1       2       0     N/A
//  Average:         5       7       0     N/A
//  Maximum:       102     126       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         2       5       0       0
//  Average:        13      17       0       0
//  Maximum:       216     248       0       0
impl std::ops::Add<AntiCircleRotor> for Motor {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group2().xyz().with_w(other[e45]),
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<AntiDipoleInversion> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        2        0        0      N/A
    // no simd        8        0        0        0
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0().with_w(self[e12345]),
            // e415, e425, e435, e321
            other.group1() + self.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            self.group1() + other.group2().xyz().with_w(other[e5]),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(other[e4]),
        )
    }
}
impl std::ops::Add<AntiDualNum> for Motor {
    type Output = MultiVector;
    fn add(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(other[e3215]),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<AntiFlatPoint> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        1        0        0      N/A
    // no simd        3        0        0        0
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(self[e12345]),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(other[e321]),
            // e235, e315, e125, e5
            (other.group0().xyz() + self.group1().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}
impl std::ops::Add<AntiFlector> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        1        0        0      N/A
    // no simd        4        0        0        0
    fn add(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(self[e12345]),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(other[e321]),
            // e235, e315, e125, e5
            self.group1() + other.group0().xyz().with_w(other[e5]),
            // e1, e2, e3, e4
            other.group1().xyz().with_w(0.0),
        )
    }
}
impl std::ops::Add<AntiLine> for Motor {
    type Output = MultiVector;
    fn add(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group1().with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<AntiMotor> for Motor {
    type Output = MultiVector;
    fn add(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group1().xyz().with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(other[e3215]),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<AntiPlane> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        1        0        0        0
    fn add(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(self[e12345]),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            self.group1().xyz().with_w(other[e5] + self[e5]),
            // e1, e2, e3, e4
            other.group0().xyz().with_w(0.0),
        )
    }
}
impl std::ops::Add<AntiScalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        1        0        0        0
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().xyz().with_w(self[e12345] + other[e12345]),
            // e235, e315, e125, e5
            self.group1(),
        )
    }
}
impl std::ops::AddAssign<AntiScalar> for Motor {
    fn add_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        *self = Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().xyz().with_w(self[e12345] + other[e12345]),
            // e235, e315, e125, e5
            self.group1(),
        );
    }
}
impl std::ops::Add<Circle> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        2        0        0      N/A
    // no simd        8        0        0        0
    fn add(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0().with_w(self[e12345]),
            // e415, e425, e435, e321
            other.group1() + self.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            self.group1() + other.group2().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}
impl std::ops::Add<CircleRotor> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd3        1        0        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        3        0        0      N/A
    //  no simd        8        0        0        0
    fn add(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0().with_w(other[e12345] + self[e12345]),
            // e415, e425, e435, e321
            other.group1() + self.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            (other.group2().xyz() + self.group1().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}
impl std::ops::Add<Dipole> for Motor {
    type Output = MultiVector;
    fn add(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group2().with_w(other[e45]),
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<DipoleInversion> for Motor {
    type Output = MultiVector;
    fn add(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group2().xyz().with_w(other[e45]),
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1().xyz(),
            // e4235, e4315, e4125, e3215
            other.group3(),
            // e1234
            other[e1234],
        )
    }
}
impl std::ops::Add<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        0        0        0
    fn add(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().xyz().with_w(other[e12345] + self[e12345]),
            // e235, e315, e125, e5
            self.group1().xyz().with_w(other[e5] + self[e5]),
        )
    }
}
impl std::ops::AddAssign<DualNum> for Motor {
    fn add_assign(&mut self, other: DualNum) {
        use crate::elements::*;
        *self = Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().xyz().with_w(other[e12345] + self[e12345]),
            // e235, e315, e125, e5
            self.group1().xyz().with_w(other[e5] + self[e5]),
        );
    }
}
impl std::ops::Add<FlatPoint> for Motor {
    type Output = MultiVector;
    fn add(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<Flector> for Motor {
    type Output = MultiVector;
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1().xyz(),
            // e4235, e4315, e4125, e3215
            other.group1(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        2        0        0      N/A
    // no simd        8        0        0        0
    fn add(self, other: Line) -> Self::Output {
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0() + other.group0().with_w(0.0),
            // e235, e315, e125, e5
            self.group1() + other.group1().with_w(0.0),
        )
    }
}
impl std::ops::AddAssign<Line> for Motor {
    fn add_assign(&mut self, other: Line) {
        *self = Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0() + other.group0().with_w(0.0),
            // e235, e315, e125, e5
            self.group1() + other.group1().with_w(0.0),
        );
    }
}
impl std::ops::Add<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        2        0        0      N/A
    // no simd        8        0        0        0
    fn add(self, other: Motor) -> Self::Output {
        Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() + self.group0(),
            // e235, e315, e125, e5
            other.group1() + self.group1(),
        )
    }
}
impl std::ops::AddAssign<Motor> for Motor {
    fn add_assign(&mut self, other: Motor) {
        *self = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() + self.group0(),
            // e235, e315, e125, e5
            other.group1() + self.group1(),
        );
    }
}
impl std::ops::Add<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd2        1        0        0      N/A
    //    simd3        2        0        0      N/A
    // Totals...
    // yes simd        4        0        0      N/A
    //  no simd        9        0        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]) + other.group0(),
            // e1, e2, e3, e4
            other.group1(),
            // e5
            self[e5] + other[e5],
            // e15, e25, e35, e45
            other.group3(),
            // e41, e42, e43
            other.group4(),
            // e23, e31, e12
            other.group5(),
            // e415, e425, e435, e321
            (self.group0().xyz() + other.group6().xyz()).with_w(other[e321]),
            // e423, e431, e412
            other.group7(),
            // e235, e315, e125
            other.group8() + self.group1().xyz(),
            // e4235, e4315, e4125, e3215
            other.group9(),
            // e1234
            other[e1234],
        )
    }
}
impl std::ops::Add<Plane> for Motor {
    type Output = MultiVector;
    fn add(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1().xyz(),
            // e4235, e4315, e4125, e3215
            other.group0(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<RoundPoint> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        1        0        0        0
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(self[e12345]),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            self.group1().xyz().with_w(self[e5] + other[e5]),
            // e1, e2, e3, e4
            other.group0(),
        )
    }
}
impl std::ops::Add<Scalar> for Motor {
    type Output = MultiVector;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<Sphere> for Motor {
    type Output = MultiVector;
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1().xyz(),
            // e4235, e4315, e4125, e3215
            other.group0(),
            // e1234
            other[e1234],
        )
    }
}
impl std::ops::Add<VersorEven> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd3        1        0        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        3        0        0      N/A
    //  no simd        8        0        0        0
    fn add(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0().xyz().with_w(self[e12345] + other[e12345]),
            // e415, e425, e435, e321
            (self.group0().xyz() + other.group1().xyz()).with_w(other[e321]),
            // e235, e315, e125, e5
            self.group1() + other.group2(),
            // e1, e2, e3, e4
            other.group3(),
        )
    }
}
impl std::ops::Add<VersorOdd> for Motor {
    type Output = MultiVector;
    fn add(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group2().xyz().with_w(other[e45]),
            // e41, e42, e43
            other.group0().xyz(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1().xyz(),
            // e4235, e4315, e4125, e3215
            other.group3(),
            // e1234
            other[e1234],
        )
    }
}
impl std::ops::BitXor<AntiCircleRotor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        2        4        0      N/A
    // Totals...
    // yes simd        5        9        0      N/A
    //  no simd        9       17        0        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<AntiCircleRotor> for Motor {
    fn bitxor_assign(&mut self, other: AntiCircleRotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2        2        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd        3       10        0      N/A
    //  no simd       10       21        0        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<AntiDualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn bitxor(self, other: AntiDualNum) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<AntiDualNum> for Motor {
    fn bitxor_assign(&mut self, other: AntiDualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatPoint> for Motor {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn bitxor(self, other: AntiFlatPoint) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<AntiFlector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd        1        8        0      N/A
    //  no simd        4       16        0        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<AntiLine> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<AntiLine> for Motor {
    fn bitxor_assign(&mut self, other: AntiLine) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd        6       14        0        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<AntiMotor> for Motor {
    fn bitxor_assign(&mut self, other: AntiMotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        8       14        0        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Circle> for Motor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        8        0        0
    fn bitxor(self, other: Circle) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<CircleRotor> for Motor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        8        0        0
    fn bitxor(self, other: CircleRotor) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Dipole> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        5       12        0        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<Dipole> for Motor {
    fn bitxor_assign(&mut self, other: Dipole) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversion> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        7        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        6        9        0      N/A
    //  no simd        6       13        0        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<DipoleInversion> for Motor {
    fn bitxor_assign(&mut self, other: DipoleInversion) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       12        0        0
    //    simd2        0        1        0      N/A
    //    simd3        3        5        0      N/A
    //    simd4        3        3        0      N/A
    // Totals...
    // yes simd       13       21        0      N/A
    //  no simd       28       41        0        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<RoundPoint> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd2        0        1        0      N/A
    //    simd3        1        1        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd       11       16        0        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<Scalar> for Motor {
    fn bitxor_assign(&mut self, other: Scalar) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Sphere> for Motor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn bitxor(self, other: Sphere) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<VersorEven> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2        2        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd        3       10        0      N/A
    //  no simd       10       21        0        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<VersorOdd> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd3        1        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       16       21        0        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<VersorOdd> for Motor {
    fn bitxor_assign(&mut self, other: VersorOdd) {
        *self = self.wedge(other);
    }
}

impl From<AntiScalar> for Motor {
    fn from(from_anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(0.0).with_w(from_anti_scalar[e12345]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        )
    }
}

impl From<DualNum> for Motor {
    fn from(from_dual_num: DualNum) -> Self {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(0.0).with_w(from_dual_num[e12345]),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(from_dual_num[e5]),
        )
    }
}

impl From<Line> for Motor {
    fn from(from_line: Line) -> Self {
        Motor::from_groups(
            // e415, e425, e435, e12345
            from_line.group0().with_w(0.0),
            // e235, e315, e125, e5
            from_line.group1().with_w(0.0),
        )
    }
}
impl std::ops::Mul<AntiCircleRotor> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       13        0        0
    //    simd2        6        8        0      N/A
    //    simd3        9       11        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd       28       35        0      N/A
    //  no simd       64       74        0        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiDipoleInversion> for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       17       24        0        0
    //    simd2        9       12        0      N/A
    //    simd3       11       12        0      N/A
    //    simd4        6        6        0      N/A
    // Totals...
    // yes simd       43       54        0      N/A
    //  no simd       92      108        0        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiDualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        1        3        0      N/A
    // no simd        4       12        0        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<AntiDualNum> for Motor {
    fn mul_assign(&mut self, other: AntiDualNum) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       12        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2        2        0      N/A
    // Totals...
    // yes simd        9       15        0      N/A
    //  no simd       13       20        0        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiFlector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        9        0        0
    //    simd2        4        6        0      N/A
    //    simd3        2        2        0      N/A
    //    simd4        5        4        0      N/A
    // Totals...
    // yes simd       16       21        0      N/A
    //  no simd       39       43        0        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiLine> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        7        9        0      N/A
    // no simd       21       27        0        0
    fn mul(self, other: AntiLine) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<AntiLine> for Motor {
    fn mul_assign(&mut self, other: AntiLine) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        8        0        0
    //    simd2        5        7        0      N/A
    //    simd3        3        3        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd       16       20        0      N/A
    //  no simd       36       39        0        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<AntiMotor> for Motor {
    fn mul_assign(&mut self, other: AntiMotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd2        2        3        0      N/A
    //    simd3        1        1        0      N/A
    //    simd4        3        3        0      N/A
    // Totals...
    // yes simd        8       10        0      N/A
    //  no simd       21       24        0        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiScalar> for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       10        0        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Circle> for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       12        0        0
    //    simd2        4        6        0      N/A
    //    simd3       10       12        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd       22       32        0      N/A
    //  no simd       52       68        0        0
    fn mul(self, other: Circle) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<CircleRotor> for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       17        0        0
    //    simd2        7        9        0      N/A
    //    simd3        9       11        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd       29       39        0      N/A
    //  no simd       60       76        0        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Dipole> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        7        0        0
    //    simd2        4        6        0      N/A
    //    simd3        9       11        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd       21       27        0      N/A
    //  no simd       55       64        0        0
    fn mul(self, other: Dipole) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<DipoleInversion> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       18        0        0
    //    simd2        8       11        0      N/A
    //    simd3       11       12        0      N/A
    //    simd4        7        7        0      N/A
    // Totals...
    // yes simd       37       48        0      N/A
    //  no simd       88      104        0        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<DualNum> for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        1        3        0      N/A
    // Totals...
    // yes simd        1        4        0      N/A
    //  no simd        4       13        0        0
    fn mul(self, other: DualNum) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<FlatPoint> for Motor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       13        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2        3        0      N/A
    // Totals...
    // yes simd        9       17        0      N/A
    //  no simd       13       24        0        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Flector> for Motor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        5        0        0
    //    simd2        2        3        0      N/A
    //    simd3        2        2        0      N/A
    //    simd4        8        7        0      N/A
    // Totals...
    // yes simd       14       17        0      N/A
    //  no simd       44       45        0        0
    fn mul(self, other: Flector) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Line> for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        5        7        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        7       10        0      N/A
    //  no simd       23       30        0        0
    fn mul(self, other: Line) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Motor> for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        7        0        0
    //    simd2        4        6        0      N/A
    //    simd3        1        1        0      N/A
    //    simd4        6        5        0      N/A
    // Totals...
    // yes simd       15       19        0      N/A
    //  no simd       39       42        0        0
    fn mul(self, other: Motor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       48       66        0        0
    //    simd2        7       11        0      N/A
    //    simd3       34       36        0      N/A
    //    simd4       13       13        0      N/A
    // Totals...
    // yes simd      102      126        0      N/A
    //  no simd      216      248        0        0
    fn mul(self, other: MultiVector) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Plane> for Motor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd2        0        2        0      N/A
    //    simd3        2        2        0      N/A
    //    simd4        5        3        0      N/A
    // Totals...
    // yes simd        7       11        0      N/A
    //  no simd       26       26        0        0
    fn mul(self, other: Plane) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<RoundPoint> for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       13        0        0
    //    simd3        2        5        0      N/A
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd       11       22        0      N/A
    //  no simd       24       44        0        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn mul(self, other: Scalar) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Scalar> for Motor {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       13        0        0
    //    simd2        0        1        0      N/A
    //    simd3        3        4        0      N/A
    //    simd4        3        3        0      N/A
    // Totals...
    // yes simd       12       21        0      N/A
    //  no simd       27       39        0        0
    fn mul(self, other: Sphere) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<VersorEven> for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       17        0        0
    //    simd2        7       11        0      N/A
    //    simd3        4        4        0      N/A
    //    simd4       19       17        0      N/A
    // Totals...
    // yes simd       38       49        0      N/A
    //  no simd      110      119        0        0
    fn mul(self, other: VersorEven) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<VersorOdd> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       14        0        0
    //    simd2        5        8        0      N/A
    //    simd3       10       10        0      N/A
    //    simd4       14       14        0      N/A
    // Totals...
    // yes simd       38       46        0      N/A
    //  no simd      105      116        0        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Neg for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn neg(self) -> Self::Output {
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Not for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn not(self) -> Self::Output {
        self.right_dual()
    }
}
impl std::ops::Sub<AntiCircleRotor> for Motor {
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
            Simd32x2::from([other[scalar] * -1.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            (other.group2().xyz() * Simd32x3::from(-1.0)).with_w(other[e45] * -1.0),
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<AntiDipoleInversion> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd2        1        0        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        7        9        0        0
    fn sub(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() * Simd32x3::from(-1.0)).with_w(self[e12345]),
            // e415, e425, e435, e321
            (self.group0().xy() - other.group1().xy()).with_zw(self[e435] - other[e435], other[e321] * -1.0),
            // e235, e315, e125, e5
            self.group1() + -other.group2().xyz().with_w(other[e5] * -1.0),
            // e1, e2, e3, e4
            (other.group3().xyz() * Simd32x3::from(-1.0)).with_w(other[e4] * -1.0),
        )
    }
}
impl std::ops::Sub<AntiDualNum> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<AntiFlatPoint> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        1        1        0      N/A
    //  no simd        4        1        0        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(self[e12345]),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(other[e321] * -1.0),
            // e235, e315, e125, e5
            self.group1() + -other.group0().xyz().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}
impl std::ops::Sub<AntiFlector> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        1        3        0      N/A
    //  no simd        4        5        0        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(self[e12345]),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(other[e321] * -1.0),
            // e235, e315, e125, e5
            self.group1() + -other.group0().xyz().with_w(other[e5] * -1.0),
            // e1, e2, e3, e4
            (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
        )
    }
}
impl std::ops::Sub<AntiLine> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn sub(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            (other.group1() * Simd32x3::from(-1.0)).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<AntiMotor> for Motor {
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
            Simd32x2::from([other[scalar] * -1.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<AntiPlane> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        1        1        0      N/A
    //  no simd        1        3        0        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(self[e12345]),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            self.group1().xyz().with_w(self[e5] - other[e5]),
            // e1, e2, e3, e4
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
        )
    }
}
impl std::ops::Sub<AntiScalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        1        0        0        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().xyz().with_w(self[e12345] - other[e12345]),
            // e235, e315, e125, e5
            self.group1(),
        )
    }
}
impl std::ops::SubAssign<AntiScalar> for Motor {
    fn sub_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        *self = Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().xyz().with_w(self[e12345] - other[e12345]),
            // e235, e315, e125, e5
            self.group1(),
        );
    }
}
impl std::ops::Sub<Circle> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        1        0        0
    //    simd2        1        0        0      N/A
    //    simd3        0        1        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        3        2        0      N/A
    //  no simd        7        4        0        0
    fn sub(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() * Simd32x3::from(-1.0)).with_w(self[e12345]),
            // e415, e425, e435, e321
            (self.group0().xy() - other.group1().xy()).with_zw(self[e435] - other[e435], other[e321] * -1.0),
            // e235, e315, e125, e5
            self.group1() + -other.group2().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}
impl std::ops::Sub<CircleRotor> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        1        0        0
    //    simd2        1        0        0      N/A
    //    simd3        0        1        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        4        2        0      N/A
    //  no simd        8        4        0        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() * Simd32x3::from(-1.0)).with_w(self[e12345] - other[e12345]),
            // e415, e425, e435, e321
            (self.group0().xy() - other.group1().xy()).with_zw(self[e435] - other[e435], other[e321] * -1.0),
            // e235, e315, e125, e5
            self.group1() + -other.group2().xyz().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}
impl std::ops::Sub<Dipole> for Motor {
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
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            (other.group2() * Simd32x3::from(-1.0)).with_w(other[e45] * -1.0),
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<DipoleInversion> for Motor {
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
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            (other.group2().xyz() * Simd32x3::from(-1.0)).with_w(other[e45] * -1.0),
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1().xyz(),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from(-1.0),
            // e1234
            other[e1234] * -1.0,
        )
    }
}
impl std::ops::Sub<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        0        0        0
    fn sub(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().xyz().with_w(self[e12345] - other[e12345]),
            // e235, e315, e125, e5
            self.group1().xyz().with_w(self[e5] - other[e5]),
        )
    }
}
impl std::ops::SubAssign<DualNum> for Motor {
    fn sub_assign(&mut self, other: DualNum) {
        use crate::elements::*;
        *self = Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().xyz().with_w(self[e12345] - other[e12345]),
            // e235, e315, e125, e5
            self.group1().xyz().with_w(self[e5] - other[e5]),
        );
    }
}
impl std::ops::Sub<FlatPoint> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from(-1.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<Flector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn sub(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from(-1.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1().xyz(),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from(-1.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        2        0        0      N/A
    // no simd        8        0        0        0
    fn sub(self, other: Line) -> Self::Output {
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0() + -other.group0().with_w(0.0),
            // e235, e315, e125, e5
            self.group1() + -other.group1().with_w(0.0),
        )
    }
}
impl std::ops::SubAssign<Line> for Motor {
    fn sub_assign(&mut self, other: Line) {
        *self = Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0() + -other.group0().with_w(0.0),
            // e235, e315, e125, e5
            self.group1() + -other.group1().with_w(0.0),
        );
    }
}
impl std::ops::Sub<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        2        0        0      N/A
    // no simd        8        0        0        0
    fn sub(self, other: Motor) -> Self::Output {
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0() - other.group0(),
            // e235, e315, e125, e5
            self.group1() - other.group1(),
        )
    }
}
impl std::ops::SubAssign<Motor> for Motor {
    fn sub_assign(&mut self, other: Motor) {
        *self = Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0() - other.group0(),
            // e235, e315, e125, e5
            self.group1() - other.group1(),
        );
    }
}
impl std::ops::Sub<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        3        0        0
    //    simd2        1        0        0      N/A
    //    simd3        1        3        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        5        9        0      N/A
    //  no simd        8       24        0        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            other.group1() * Simd32x4::from(-1.0),
            // e5
            self[e5] - other[e5],
            // e15, e25, e35, e45
            other.group3() * Simd32x4::from(-1.0),
            // e41, e42, e43
            other.group4() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group5() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            (self.group0().xy() - other.group6().xy()).with_zw(self[e435] - other[e435], other[e321] * -1.0),
            // e423, e431, e412
            other.group7() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1().xyz() - other.group8(),
            // e4235, e4315, e4125, e3215
            other.group9() * Simd32x4::from(-1.0),
            // e1234
            other[e1234] * -1.0,
        )
    }
}
impl std::ops::Sub<Plane> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn sub(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1().xyz(),
            // e4235, e4315, e4125, e3215
            other.group0() * Simd32x4::from(-1.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<RoundPoint> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1        1        0      N/A
    //  no simd        1        4        0        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(self[e12345]),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            self.group1().xyz().with_w(self[e5] - other[e5]),
            // e1, e2, e3, e4
            other.group0() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Sub<Scalar> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<Sphere> for Motor {
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
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1().xyz(),
            // e4235, e4315, e4125, e3215
            other.group0() * Simd32x4::from(-1.0),
            // e1234
            other[e1234] * -1.0,
        )
    }
}
impl std::ops::Sub<VersorEven> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        1        0        0
    //    simd2        1        0        0      N/A
    //    simd3        0        1        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd        4        3        0      N/A
    //  no simd        8        8        0        0
    fn sub(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e12345] - other[e12345]),
            // e415, e425, e435, e321
            (self.group0().xy() - other.group1().xy()).with_zw(self[e435] - other[e435], other[e321] * -1.0),
            // e235, e315, e125, e5
            self.group1() - other.group2(),
            // e1, e2, e3, e4
            other.group3() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Sub<VersorOdd> for Motor {
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
            Simd32x2::from([other[scalar] * -1.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            (other.group2().xyz() * Simd32x3::from(-1.0)).with_w(other[e45] * -1.0),
            // e41, e42, e43
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1().xyz(),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from(-1.0),
            // e1234
            other[e1234] * -1.0,
        )
    }
}

impl TryFrom<AntiDipoleInversion> for Motor {
    type Error = String;
    fn try_from(anti_dipole_inversion: AntiDipoleInversion) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversion do not fit into Motor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(Motor::from_groups(
            // e415, e425, e435, e12345
            anti_dipole_inversion.group1().xyz().with_w(0.0),
            // e235, e315, e125, e5
            anti_dipole_inversion.group2().xyz().with_w(anti_dipole_inversion[e5]),
        ))
    }
}

impl TryFrom<AntiFlatPoint> for Motor {
    type Error = String;
    fn try_from(anti_flat_point: AntiFlatPoint) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_flat_point[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiFlatPoint do not fit into Motor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            anti_flat_point.group0().xyz().with_w(0.0),
        ))
    }
}

impl TryFrom<AntiFlector> for Motor {
    type Error = String;
    fn try_from(anti_flector: AntiFlector) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_flector[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiFlector do not fit into Motor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            anti_flector.group0().xyz().with_w(anti_flector[e5]),
        ))
    }
}

impl TryFrom<AntiPlane> for Motor {
    type Error = String;
    fn try_from(anti_plane: AntiPlane) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_plane[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_plane[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_plane[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiPlane do not fit into Motor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(anti_plane[e5]),
        ))
    }
}

impl TryFrom<Circle> for Motor {
    type Error = String;
    fn try_from(circle: Circle) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Circle do not fit into Motor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(Motor::from_groups(
            // e415, e425, e435, e12345
            circle.group1().xyz().with_w(0.0),
            // e235, e315, e125, e5
            circle.group2().with_w(0.0),
        ))
    }
}

impl TryFrom<CircleRotor> for Motor {
    type Error = String;
    fn try_from(circle_rotor: CircleRotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotor do not fit into Motor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(Motor::from_groups(
            // e415, e425, e435, e12345
            circle_rotor.group1().xyz().with_w(circle_rotor[e12345]),
            // e235, e315, e125, e5
            circle_rotor.group2().xyz().with_w(0.0),
        ))
    }
}

impl TryFrom<MultiVector> for Motor {
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
            let mut error = "Elements from MultiVector do not fit into Motor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(Motor::from_groups(
            // e415, e425, e435, e12345
            multi_vector.group6().xyz().with_w(multi_vector[e12345]),
            // e235, e315, e125, e5
            multi_vector.group8().with_w(multi_vector[e5]),
        ))
    }
}

impl TryFrom<RoundPoint> for Motor {
    type Error = String;
    fn try_from(round_point: RoundPoint) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = round_point[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = round_point[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = round_point[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = round_point[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from RoundPoint do not fit into Motor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(round_point[e5]),
        ))
    }
}

impl TryFrom<VersorEven> for Motor {
    type Error = String;
    fn try_from(versor_even: VersorEven) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[14];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[15];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEven do not fit into Motor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(Motor::from_groups(
            // e415, e425, e435, e12345
            versor_even.group1().xyz().with_w(versor_even[e12345]),
            // e235, e315, e125, e5
            versor_even.group2(),
        ))
    }
}
