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
// Total Implementations: 113
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         3       2       0     N/A
//  Average:        13      16       0     N/A
//  Maximum:       234     262       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         6       5       0       0
//  Average:        30      35       0       0
//  Maximum:       466     494       0       0
impl std::ops::Add<AntiCircleRotor> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group2().xyz().with_w(other[e45]),
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<AntiDipoleInversion> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        4        0        0      N/A
    // no simd       16        0        0        0
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + other.group0().with_w(0.0),
            // e415, e425, e435, e321
            other.group1() + self.group1(),
            // e235, e315, e125, e5
            self.group2() + other.group2().xyz().with_w(other[e5]),
            // e1, e2, e3, e4
            self.group3() + other.group3().xyz().with_w(other[e4]),
        )
    }
}
impl std::ops::AddAssign<AntiDipoleInversion> for VersorEven {
    fn add_assign(&mut self, other: AntiDipoleInversion) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + other.group0().with_w(0.0),
            // e415, e425, e435, e321
            other.group1() + self.group1(),
            // e235, e315, e125, e5
            self.group2() + other.group2().xyz().with_w(other[e5]),
            // e1, e2, e3, e4
            self.group3() + other.group3().xyz().with_w(other[e4]),
        );
    }
}
impl std::ops::Add<AntiDualNum> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
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
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(other[e3215]),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<AntiFlatPoint> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd3        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        4        0        0        0
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1().xyz().with_w(other[e321] + self[e321]),
            // e235, e315, e125, e5
            (other.group0().xyz() + self.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group3(),
        )
    }
}
impl std::ops::AddAssign<AntiFlatPoint> for VersorEven {
    fn add_assign(&mut self, other: AntiFlatPoint) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1().xyz().with_w(other[e321] + self[e321]),
            // e235, e315, e125, e5
            (other.group0().xyz() + self.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<AntiFlector> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd3        1        0        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        3        0        0      N/A
    //  no simd        8        0        0        0
    fn add(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1().xyz().with_w(other[e321] + self[e321]),
            // e235, e315, e125, e5
            self.group2() + other.group0().xyz().with_w(other[e5]),
            // e1, e2, e3, e4
            (other.group1().xyz() + self.group3().xyz()).with_w(self[e4]),
        )
    }
}
impl std::ops::AddAssign<AntiFlector> for VersorEven {
    fn add_assign(&mut self, other: AntiFlector) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1().xyz().with_w(other[e321] + self[e321]),
            // e235, e315, e125, e5
            self.group2() + other.group0().xyz().with_w(other[e5]),
            // e1, e2, e3, e4
            (other.group1().xyz() + self.group3().xyz()).with_w(self[e4]),
        );
    }
}
impl std::ops::Add<AntiLine> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
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
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<AntiMotor> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
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
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(other[e3215]),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<AntiPlane> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd3        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        4        0        0        0
    fn add(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2().xyz().with_w(other[e5] + self[e5]),
            // e1, e2, e3, e4
            (other.group0().xyz() + self.group3().xyz()).with_w(self[e4]),
        )
    }
}
impl std::ops::AddAssign<AntiPlane> for VersorEven {
    fn add_assign(&mut self, other: AntiPlane) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2().xyz().with_w(other[e5] + self[e5]),
            // e1, e2, e3, e4
            (other.group0().xyz() + self.group3().xyz()).with_w(self[e4]),
        );
    }
}
impl std::ops::Add<AntiScalar> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        1        0        0        0
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(self[e12345] + other[e12345]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        )
    }
}
impl std::ops::AddAssign<AntiScalar> for VersorEven {
    fn add_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(self[e12345] + other[e12345]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<Circle> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        3        0        0      N/A
    // no simd       12        0        0        0
    fn add(self, other: Circle) -> Self::Output {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + other.group0().with_w(0.0),
            // e415, e425, e435, e321
            other.group1() + self.group1(),
            // e235, e315, e125, e5
            self.group2() + other.group2().with_w(0.0),
            // e1, e2, e3, e4
            self.group3(),
        )
    }
}
impl std::ops::AddAssign<Circle> for VersorEven {
    fn add_assign(&mut self, other: Circle) {
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + other.group0().with_w(0.0),
            // e415, e425, e435, e321
            other.group1() + self.group1(),
            // e235, e315, e125, e5
            self.group2() + other.group2().with_w(0.0),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<CircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd3        2        0        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        4        0        0      N/A
    //  no simd       11        0        0        0
    fn add(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() + self.group0().xyz()).with_w(other[e12345] + self[e12345]),
            // e415, e425, e435, e321
            other.group1() + self.group1(),
            // e235, e315, e125, e5
            (other.group2().xyz() + self.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group3(),
        )
    }
}
impl std::ops::AddAssign<CircleRotor> for VersorEven {
    fn add_assign(&mut self, other: CircleRotor) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() + self.group0().xyz()).with_w(other[e12345] + self[e12345]),
            // e415, e425, e435, e321
            other.group1() + self.group1(),
            // e235, e315, e125, e5
            (other.group2().xyz() + self.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<Dipole> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group2().with_w(other[e45]),
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<DipoleInversion> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group2().xyz().with_w(other[e45]),
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            other.group3(),
            // e1234
            other[e1234],
        )
    }
}
impl std::ops::Add<DualNum> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        0        0        0
    fn add(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345] + self[e12345]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2().xyz().with_w(other[e5] + self[e5]),
            // e1, e2, e3, e4
            self.group3(),
        )
    }
}
impl std::ops::AddAssign<DualNum> for VersorEven {
    fn add_assign(&mut self, other: DualNum) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345] + self[e12345]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2().xyz().with_w(other[e5] + self[e5]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<FlatPoint> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
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
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<Flector> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
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
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            other.group1(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<Line> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        2        0        0      N/A
    // no simd        8        0        0        0
    fn add(self, other: Line) -> Self::Output {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1() + other.group0().with_w(0.0),
            // e235, e315, e125, e5
            self.group2() + other.group1().with_w(0.0),
            // e1, e2, e3, e4
            self.group3(),
        )
    }
}
impl std::ops::AddAssign<Line> for VersorEven {
    fn add_assign(&mut self, other: Line) {
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1() + other.group0().with_w(0.0),
            // e235, e315, e125, e5
            self.group2() + other.group1().with_w(0.0),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<Motor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd3        1        0        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        3        0        0      N/A
    //  no simd        8        0        0        0
    fn add(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345] + self[e12345]),
            // e415, e425, e435, e321
            (other.group0().xyz() + self.group1().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            other.group1() + self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        )
    }
}
impl std::ops::AddAssign<Motor> for VersorEven {
    fn add_assign(&mut self, other: Motor) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345] + self[e12345]),
            // e415, e425, e435, e321
            (other.group0().xyz() + self.group1().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            other.group1() + self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<MultiVector> for VersorEven {
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
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]) + other.group0(),
            // e1, e2, e3, e4
            other.group1() + self.group3(),
            // e5
            self[e5] + other[e5],
            // e15, e25, e35, e45
            other.group3(),
            // e41, e42, e43
            other.group4(),
            // e23, e31, e12
            other.group5(),
            // e415, e425, e435, e321
            other.group6() + self.group1(),
            // e423, e431, e412
            other.group7() + self.group0().xyz(),
            // e235, e315, e125
            other.group8() + self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            other.group9(),
            // e1234
            other[e1234],
        )
    }
}
impl std::ops::Add<Plane> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
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
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            other.group0(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<RoundPoint> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        5        0        0        0
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2().xyz().with_w(self[e5] + other[e5]),
            // e1, e2, e3, e4
            other.group0() + self.group3(),
        )
    }
}
impl std::ops::AddAssign<RoundPoint> for VersorEven {
    fn add_assign(&mut self, other: RoundPoint) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2().xyz().with_w(self[e5] + other[e5]),
            // e1, e2, e3, e4
            other.group0() + self.group3(),
        );
    }
}
impl std::ops::Add<Scalar> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
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
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<Sphere> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
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
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            other.group0(),
            // e1234
            other[e1234],
        )
    }
}
impl std::ops::Add<VersorEven> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        4        0        0      N/A
    // no simd       16        0        0        0
    fn add(self, other: VersorEven) -> Self::Output {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() + self.group0(),
            // e415, e425, e435, e321
            other.group1() + self.group1(),
            // e235, e315, e125, e5
            other.group2() + self.group2(),
            // e1, e2, e3, e4
            other.group3() + self.group3(),
        )
    }
}
impl std::ops::AddAssign<VersorEven> for VersorEven {
    fn add_assign(&mut self, other: VersorEven) {
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() + self.group0(),
            // e415, e425, e435, e321
            other.group1() + self.group1(),
            // e235, e315, e125, e5
            other.group2() + self.group2(),
            // e1, e2, e3, e4
            other.group3() + self.group3(),
        );
    }
}
impl std::ops::Add<VersorOdd> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group2().xyz().with_w(other[e45]),
            // e41, e42, e43
            other.group0().xyz(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            other.group3(),
            // e1234
            other[e1234],
        )
    }
}
impl std::ops::BitXor<AntiCircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       13        0        0
    //    simd3        8       10        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       15       24        0      N/A
    //  no simd       31       47        0        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<AntiCircleRotor> for VersorEven {
    fn bitxor_assign(&mut self, other: AntiCircleRotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       15        0        0
    //    simd2        1        3        0      N/A
    //    simd3        4        6        0      N/A
    //    simd4        5        5        0      N/A
    // Totals...
    // yes simd       18       29        0      N/A
    //  no simd       42       59        0        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<AntiDualNum> for VersorEven {
    type Output = VersorEven;
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
impl std::ops::BitXorAssign<AntiDualNum> for VersorEven {
    fn bitxor_assign(&mut self, other: AntiDualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatPoint> for VersorEven {
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
impl std::ops::BitXor<AntiFlector> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       12        0        0
    //    simd2        1        3        0      N/A
    //    simd3        1        2        0      N/A
    //    simd4        4        4        0      N/A
    // Totals...
    // yes simd       12       21        0      N/A
    //  no simd       27       40        0        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<AntiLine> for VersorEven {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd        8       18        0        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<AntiMotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       14        0        0
    //    simd3        2        4        0      N/A
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd       13       21        0      N/A
    //  no simd       23       38        0        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<AntiMotor> for VersorEven {
    fn bitxor_assign(&mut self, other: AntiMotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        5        0        0
    //    simd2        1        3        0      N/A
    //    simd3        1        3        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        6       13        0      N/A
    //  no simd       18       28        0        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Circle> for VersorEven {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd2        1        2        0      N/A
    //    simd3        2        2        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        8       10        0      N/A
    //  no simd       16       16        0        0
    fn bitxor(self, other: Circle) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<CircleRotor> for VersorEven {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd2        1        2        0      N/A
    //    simd3        2        2        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        8       10        0      N/A
    //  no simd       16       16        0        0
    fn bitxor(self, other: CircleRotor) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Dipole> for VersorEven {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        6        9        0      N/A
    // Totals...
    // yes simd        9       13        0      N/A
    //  no simd       21       31        0        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<DipoleInversion> for VersorEven {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       12        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4        6        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd       15       21        0      N/A
    //  no simd       29       40        0        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<DualNum> for VersorEven {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<FlatPoint> for VersorEven {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd2        1        2        0      N/A
    //    simd3        0        1        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd       11       13        0        0
    fn bitxor(self, other: FlatPoint) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Flector> for VersorEven {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd2        1        2        0      N/A
    //    simd3        0        1        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        7       10        0      N/A
    //  no simd       14       17        0        0
    fn bitxor(self, other: Flector) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Line> for VersorEven {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        3        0      N/A
    // no simd        6        9        0        0
    fn bitxor(self, other: Line) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Motor> for VersorEven {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd2        1        2        0      N/A
    //    simd3        2        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd        9       17        0        0
    fn bitxor(self, other: Motor) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<MultiVector> for VersorEven {
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
    fn bitxor(self, other: MultiVector) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Plane> for VersorEven {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn bitxor(self, other: Plane) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<RoundPoint> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd2        0        2        0      N/A
    //    simd3        3        5        0      N/A
    //    simd4        3        3        0      N/A
    // Totals...
    // yes simd        6       15        0      N/A
    //  no simd       21       36        0        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Scalar> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<Scalar> for VersorEven {
    fn bitxor_assign(&mut self, other: Scalar) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Sphere> for VersorEven {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4        5        0        0
    fn bitxor(self, other: Sphere) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<VersorEven> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       11        0        0
    //    simd2        1        2        0      N/A
    //    simd3        4        6        0      N/A
    //    simd4        5        6        0      N/A
    // Totals...
    // yes simd       18       25        0      N/A
    //  no simd       42       57        0        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<VersorOdd> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       14       20        0        0
    //    simd2        1        2        0      N/A
    //    simd3        4        5        0      N/A
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd       22       31        0      N/A
    //  no simd       40       55        0        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<VersorOdd> for VersorEven {
    fn bitxor_assign(&mut self, other: VersorOdd) {
        *self = self.wedge(other);
    }
}

impl From<AntiDipoleInversion> for VersorEven {
    fn from(from_anti_dipole_inversion: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_anti_dipole_inversion.group0().with_w(0.0),
            // e415, e425, e435, e321
            from_anti_dipole_inversion.group1(),
            // e235, e315, e125, e5
            from_anti_dipole_inversion.group2().xyz().with_w(from_anti_dipole_inversion[e5]),
            // e1, e2, e3, e4
            from_anti_dipole_inversion.group3().xyz().with_w(from_anti_dipole_inversion[e4]),
        )
    }
}

impl From<AntiFlatPoint> for VersorEven {
    fn from(from_anti_flat_point: AntiFlatPoint) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flat_point[e321]),
            // e235, e315, e125, e5
            from_anti_flat_point.group0().xyz().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiFlector> for VersorEven {
    fn from(from_anti_flector: AntiFlector) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flector[e321]),
            // e235, e315, e125, e5
            from_anti_flector.group0().xyz().with_w(from_anti_flector[e5]),
            // e1, e2, e3, e4
            from_anti_flector.group1().xyz().with_w(0.0),
        )
    }
}

impl From<AntiPlane> for VersorEven {
    fn from(from_anti_plane: AntiPlane) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(from_anti_plane[e5]),
            // e1, e2, e3, e4
            from_anti_plane.group0().xyz().with_w(0.0),
        )
    }
}

impl From<AntiScalar> for VersorEven {
    fn from(from_anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(from_anti_scalar[e12345]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<Circle> for VersorEven {
    fn from(from_circle: Circle) -> Self {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_circle.group0().with_w(0.0),
            // e415, e425, e435, e321
            from_circle.group1(),
            // e235, e315, e125, e5
            from_circle.group2().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<CircleRotor> for VersorEven {
    fn from(from_circle_rotor: CircleRotor) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_circle_rotor.group0().with_w(from_circle_rotor[e12345]),
            // e415, e425, e435, e321
            from_circle_rotor.group1(),
            // e235, e315, e125, e5
            from_circle_rotor.group2().xyz().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<DualNum> for VersorEven {
    fn from(from_dual_num: DualNum) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(from_dual_num[e12345]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(from_dual_num[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<Line> for VersorEven {
    fn from(from_line: Line) -> Self {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            from_line.group0().with_w(0.0),
            // e235, e315, e125, e5
            from_line.group1().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<Motor> for VersorEven {
    fn from(from_motor: Motor) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(from_motor[e12345]),
            // e415, e425, e435, e321
            from_motor.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            from_motor.group1(),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<RoundPoint> for VersorEven {
    fn from(from_round_point: RoundPoint) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(from_round_point[e5]),
            // e1, e2, e3, e4
            from_round_point.group0(),
        )
    }
}
impl std::ops::Mul<AntiCircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       17       25        0        0
    //    simd2       14       18        0      N/A
    //    simd3       21       21        0      N/A
    //    simd4        5        5        0      N/A
    // Totals...
    // yes simd       57       69        0      N/A
    //  no simd      128      144        0        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<AntiCircleRotor> for VersorEven {
    fn mul_assign(&mut self, other: AntiCircleRotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       40       49        0        0
    //    simd2       14       18        0      N/A
    //    simd3       24       24        0      N/A
    //    simd4       18       18        0      N/A
    // Totals...
    // yes simd       96      109        0      N/A
    //  no simd      212      229        0        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiDualNum> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        1        3        0      N/A
    //    simd4        3        5        0      N/A
    // Totals...
    // yes simd        5       11        0      N/A
    //  no simd       16       32        0        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<AntiDualNum> for VersorEven {
    fn mul_assign(&mut self, other: AntiDualNum) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        8        0        0
    //    simd3        6        8        0      N/A
    //    simd4        7        8        0      N/A
    // Totals...
    // yes simd       18       24        0      N/A
    //  no simd       51       64        0        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiFlector> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       16        0        0
    //    simd2        4        7        0      N/A
    //    simd3        8        8        0      N/A
    //    simd4       17       17        0      N/A
    // Totals...
    // yes simd       39       48        0      N/A
    //  no simd      110      122        0        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiLine> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3       20       24        0      N/A
    // Totals...
    // yes simd       22       27        0      N/A
    //  no simd       62       75        0        0
    fn mul(self, other: AntiLine) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<AntiLine> for VersorEven {
    fn mul_assign(&mut self, other: AntiLine) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       20        0        0
    //    simd2        9       13        0      N/A
    //    simd3       12       12        0      N/A
    //    simd4        8        7        0      N/A
    // Totals...
    // yes simd       42       52        0      N/A
    //  no simd       99      110        0        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<AntiMotor> for VersorEven {
    fn mul_assign(&mut self, other: AntiMotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd2        2        3        0      N/A
    //    simd3        2        2        0      N/A
    //    simd4        9       11        0      N/A
    // Totals...
    // yes simd       15       19        0      N/A
    //  no simd       48       59        0        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiScalar> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        6        0      N/A
    // Totals...
    // yes simd        0        8        0      N/A
    //  no simd        0       26        0        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Circle> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       21       28        0        0
    //    simd2        4        7        0      N/A
    //    simd3       25       26        0      N/A
    //    simd4        7        7        0      N/A
    // Totals...
    // yes simd       57       68        0      N/A
    //  no simd      132      148        0        0
    fn mul(self, other: Circle) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<CircleRotor> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       22       30        0        0
    //    simd2        8       12        0      N/A
    //    simd3       22       22        0      N/A
    //    simd4       10       10        0      N/A
    // Totals...
    // yes simd       62       74        0      N/A
    //  no simd      144      160        0        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Dipole> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       14        0        0
    //    simd2        8       12        0      N/A
    //    simd3       22       22        0      N/A
    //    simd4        8        6        0      N/A
    // Totals...
    // yes simd       46       54        0      N/A
    //  no simd      122      128        0        0
    fn mul(self, other: Dipole) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Dipole> for VersorEven {
    fn mul_assign(&mut self, other: Dipole) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       43       51        0        0
    //    simd2       24       28        0      N/A
    //    simd3       23       23        0      N/A
    //    simd4        9        9        0      N/A
    // Totals...
    // yes simd       99      111        0      N/A
    //  no simd      196      212        0        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<DipoleInversion> for VersorEven {
    fn mul_assign(&mut self, other: DipoleInversion) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        5        0        0
    //    simd3        3        8        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd        5       14        0      N/A
    //  no simd       14       33        0        0
    fn mul(self, other: DualNum) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<FlatPoint> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       15        0        0
    //    simd2        2        4        0      N/A
    //    simd3        3        3        0      N/A
    //    simd4        7        7        0      N/A
    // Totals...
    // yes simd       21       29        0      N/A
    //  no simd       50       60        0        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<FlatPoint> for VersorEven {
    fn mul_assign(&mut self, other: FlatPoint) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       19        0        0
    //    simd2        7       10        0      N/A
    //    simd3        3        4        0      N/A
    //    simd4       20       18        0      N/A
    // Totals...
    // yes simd       43       51        0      N/A
    //  no simd      116      123        0        0
    fn mul(self, other: Flector) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Flector> for VersorEven {
    fn mul_assign(&mut self, other: Flector) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       10        0        0
    //    simd3       19       23        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd       27       34        0      N/A
    //  no simd       68       83        0        0
    fn mul(self, other: Line) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Motor> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       11        0        0
    //    simd2        2        4        0      N/A
    //    simd3        9        9        0      N/A
    //    simd4       18       19        0      N/A
    // Totals...
    // yes simd       34       43        0      N/A
    //  no simd      108      122        0        0
    fn mul(self, other: Motor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<MultiVector> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32      122      146        0        0
    //    simd2       12       20        0      N/A
    //    simd3       80       76        0      N/A
    //    simd4       20       20        0      N/A
    // Totals...
    // yes simd      234      262        0      N/A
    //  no simd      466      494        0        0
    fn mul(self, other: MultiVector) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Plane> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        9        0        0
    //    simd2        3        7        0      N/A
    //    simd3        2        2        0      N/A
    //    simd4       11        7        0      N/A
    // Totals...
    // yes simd       19       25        0      N/A
    //  no simd       59       57        0        0
    fn mul(self, other: Plane) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Plane> for VersorEven {
    fn mul_assign(&mut self, other: Plane) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       12        0        0
    //    simd2        2        5        0      N/A
    //    simd3        6        7        0      N/A
    //    simd4        9        8        0      N/A
    // Totals...
    // yes simd       21       32        0      N/A
    //  no simd       62       75        0        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Scalar> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
    fn mul(self, other: Scalar) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Scalar> for VersorEven {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       14        0        0
    //    simd2        1        5        0      N/A
    //    simd3        6        6        0      N/A
    //    simd4       10        9        0      N/A
    // Totals...
    // yes simd       22       34        0      N/A
    //  no simd       65       78        0        0
    fn mul(self, other: Sphere) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Sphere> for VersorEven {
    fn mul_assign(&mut self, other: Sphere) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       54       62        0        0
    //    simd2       24       28        0      N/A
    //    simd3       18       18        0      N/A
    //    simd4       18       18        0      N/A
    // Totals...
    // yes simd      114      126        0      N/A
    //  no simd      228      244        0        0
    fn mul(self, other: VersorEven) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<VersorOdd> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       47       55        0        0
    //    simd2       30       34        0      N/A
    //    simd3       15       15        0      N/A
    //    simd4       15       15        0      N/A
    // Totals...
    // yes simd      107      119        0      N/A
    //  no simd      212      228        0        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<VersorOdd> for VersorEven {
    fn mul_assign(&mut self, other: VersorOdd) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Neg for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
    fn neg(self) -> Self::Output {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from(-1.0),
            // e1, e2, e3, e4
            self.group3() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Not for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        4        0        0
    fn not(self) -> Self::Output {
        self.right_dual()
    }
}
impl std::ops::Sub<AntiCircleRotor> for VersorEven {
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
            self.group3(),
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
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<AntiDipoleInversion> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        4        0        0      N/A
    // Totals...
    // yes simd        4        2        0      N/A
    //  no simd       16        2        0        0
    fn sub(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + -other.group0().with_w(0.0),
            // e415, e425, e435, e321
            self.group1() - other.group1(),
            // e235, e315, e125, e5
            self.group2() + -other.group2().xyz().with_w(other[e5] * -1.0),
            // e1, e2, e3, e4
            self.group3() + -other.group3().xyz().with_w(other[e4] * -1.0),
        )
    }
}
impl std::ops::SubAssign<AntiDipoleInversion> for VersorEven {
    fn sub_assign(&mut self, other: AntiDipoleInversion) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + -other.group0().with_w(0.0),
            // e415, e425, e435, e321
            self.group1() - other.group1(),
            // e235, e315, e125, e5
            self.group2() + -other.group2().xyz().with_w(other[e5] * -1.0),
            // e1, e2, e3, e4
            self.group3() + -other.group3().xyz().with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::Sub<AntiDualNum> for VersorEven {
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
            self.group3(),
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
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<AntiFlatPoint> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        5        0        0        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1().xyz().with_w(self[e321] - other[e321]),
            // e235, e315, e125, e5
            self.group2() + -other.group0().xyz().with_w(0.0),
            // e1, e2, e3, e4
            self.group3(),
        )
    }
}
impl std::ops::SubAssign<AntiFlatPoint> for VersorEven {
    fn sub_assign(&mut self, other: AntiFlatPoint) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1().xyz().with_w(self[e321] - other[e321]),
            // e235, e315, e125, e5
            self.group2() + -other.group0().xyz().with_w(0.0),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<AntiFlector> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        1        0        0
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        3        1        0      N/A
    //  no simd        9        1        0        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1().xyz().with_w(self[e321] - other[e321]),
            // e235, e315, e125, e5
            self.group2() + -other.group0().xyz().with_w(other[e5] * -1.0),
            // e1, e2, e3, e4
            self.group3() + -other.group1().xyz().with_w(0.0),
        )
    }
}
impl std::ops::SubAssign<AntiFlector> for VersorEven {
    fn sub_assign(&mut self, other: AntiFlector) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1().xyz().with_w(self[e321] - other[e321]),
            // e235, e315, e125, e5
            self.group2() + -other.group0().xyz().with_w(other[e5] * -1.0),
            // e1, e2, e3, e4
            self.group3() + -other.group1().xyz().with_w(0.0),
        );
    }
}
impl std::ops::Sub<AntiLine> for VersorEven {
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
            self.group3(),
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
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<AntiMotor> for VersorEven {
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
            self.group3(),
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
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<AntiPlane> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        5        0        0        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2().xyz().with_w(self[e5] - other[e5]),
            // e1, e2, e3, e4
            self.group3() + -other.group0().xyz().with_w(0.0),
        )
    }
}
impl std::ops::SubAssign<AntiPlane> for VersorEven {
    fn sub_assign(&mut self, other: AntiPlane) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2().xyz().with_w(self[e5] - other[e5]),
            // e1, e2, e3, e4
            self.group3() + -other.group0().xyz().with_w(0.0),
        );
    }
}
impl std::ops::Sub<AntiScalar> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        1        0        0        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(self[e12345] - other[e12345]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        )
    }
}
impl std::ops::SubAssign<AntiScalar> for VersorEven {
    fn sub_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(self[e12345] - other[e12345]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<Circle> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        3        0        0      N/A
    // no simd       12        0        0        0
    fn sub(self, other: Circle) -> Self::Output {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + -other.group0().with_w(0.0),
            // e415, e425, e435, e321
            self.group1() - other.group1(),
            // e235, e315, e125, e5
            self.group2() + -other.group2().with_w(0.0),
            // e1, e2, e3, e4
            self.group3(),
        )
    }
}
impl std::ops::SubAssign<Circle> for VersorEven {
    fn sub_assign(&mut self, other: Circle) {
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + -other.group0().with_w(0.0),
            // e415, e425, e435, e321
            self.group1() - other.group1(),
            // e235, e315, e125, e5
            self.group2() + -other.group2().with_w(0.0),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<CircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        3        1        0      N/A
    //  no simd       12        1        0        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + -other.group0().with_w(other[e12345] * -1.0),
            // e415, e425, e435, e321
            self.group1() - other.group1(),
            // e235, e315, e125, e5
            self.group2() + -other.group2().xyz().with_w(0.0),
            // e1, e2, e3, e4
            self.group3(),
        )
    }
}
impl std::ops::SubAssign<CircleRotor> for VersorEven {
    fn sub_assign(&mut self, other: CircleRotor) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + -other.group0().with_w(other[e12345] * -1.0),
            // e415, e425, e435, e321
            self.group1() - other.group1(),
            // e235, e315, e125, e5
            self.group2() + -other.group2().xyz().with_w(0.0),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<Dipole> for VersorEven {
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
            self.group3(),
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
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<DipoleInversion> for VersorEven {
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
            self.group3(),
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
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from(-1.0),
            // e1234
            other[e1234] * -1.0,
        )
    }
}
impl std::ops::Sub<DualNum> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        0        0        0
    fn sub(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(self[e12345] - other[e12345]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2().xyz().with_w(self[e5] - other[e5]),
            // e1, e2, e3, e4
            self.group3(),
        )
    }
}
impl std::ops::SubAssign<DualNum> for VersorEven {
    fn sub_assign(&mut self, other: DualNum) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(self[e12345] - other[e12345]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2().xyz().with_w(self[e5] - other[e5]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<FlatPoint> for VersorEven {
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
            self.group3(),
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
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<Flector> for VersorEven {
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
            self.group3(),
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
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from(-1.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<Line> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        2        0        0      N/A
    // no simd        8        0        0        0
    fn sub(self, other: Line) -> Self::Output {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1() + -other.group0().with_w(0.0),
            // e235, e315, e125, e5
            self.group2() + -other.group1().with_w(0.0),
            // e1, e2, e3, e4
            self.group3(),
        )
    }
}
impl std::ops::SubAssign<Line> for VersorEven {
    fn sub_assign(&mut self, other: Line) {
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1() + -other.group0().with_w(0.0),
            // e235, e315, e125, e5
            self.group2() + -other.group1().with_w(0.0),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<Motor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        3        0        0      N/A
    //  no simd        9        0        0        0
    fn sub(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(self[e12345] - other[e12345]),
            // e415, e425, e435, e321
            self.group1() + -other.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            self.group2() - other.group1(),
            // e1, e2, e3, e4
            self.group3(),
        )
    }
}
impl std::ops::SubAssign<Motor> for VersorEven {
    fn sub_assign(&mut self, other: Motor) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(self[e12345] - other[e12345]),
            // e415, e425, e435, e321
            self.group1() + -other.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            self.group2() - other.group1(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<MultiVector> for VersorEven {
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
            Simd32x2::from([other[scalar] * -1.0, self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            self.group3() - other.group1(),
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
            self.group0().xyz() - other.group7(),
            // e235, e315, e125
            self.group2().xyz() - other.group8(),
            // e4235, e4315, e4125, e3215
            other.group9() * Simd32x4::from(-1.0),
            // e1234
            other[e1234] * -1.0,
        )
    }
}
impl std::ops::Sub<Plane> for VersorEven {
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
            self.group3(),
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
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            other.group0() * Simd32x4::from(-1.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<RoundPoint> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        5        0        0        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2().xyz().with_w(self[e5] - other[e5]),
            // e1, e2, e3, e4
            self.group3() - other.group0(),
        )
    }
}
impl std::ops::SubAssign<RoundPoint> for VersorEven {
    fn sub_assign(&mut self, other: RoundPoint) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2().xyz().with_w(self[e5] - other[e5]),
            // e1, e2, e3, e4
            self.group3() - other.group0(),
        );
    }
}
impl std::ops::Sub<Scalar> for VersorEven {
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
            self.group3(),
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
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<Sphere> for VersorEven {
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
            self.group3(),
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
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            other.group0() * Simd32x4::from(-1.0),
            // e1234
            other[e1234] * -1.0,
        )
    }
}
impl std::ops::Sub<VersorEven> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        4        0        0      N/A
    // no simd       16        0        0        0
    fn sub(self, other: VersorEven) -> Self::Output {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() - other.group0(),
            // e415, e425, e435, e321
            self.group1() - other.group1(),
            // e235, e315, e125, e5
            self.group2() - other.group2(),
            // e1, e2, e3, e4
            self.group3() - other.group3(),
        )
    }
}
impl std::ops::SubAssign<VersorEven> for VersorEven {
    fn sub_assign(&mut self, other: VersorEven) {
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() - other.group0(),
            // e415, e425, e435, e321
            self.group1() - other.group1(),
            // e235, e315, e125, e5
            self.group2() - other.group2(),
            // e1, e2, e3, e4
            self.group3() - other.group3(),
        );
    }
}
impl std::ops::Sub<VersorOdd> for VersorEven {
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
            self.group3(),
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
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from(-1.0),
            // e1234
            other[e1234] * -1.0,
        )
    }
}

impl TryFrom<MultiVector> for VersorEven {
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
            let mut error = "Elements from MultiVector do not fit into VersorEven { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEven::from_groups(
            // e423, e431, e412, e12345
            multi_vector.group7().with_w(multi_vector[e12345]),
            // e415, e425, e435, e321
            multi_vector.group6(),
            // e235, e315, e125, e5
            multi_vector.group8().with_w(multi_vector[e5]),
            // e1, e2, e3, e4
            multi_vector.group1(),
        ))
    }
}
