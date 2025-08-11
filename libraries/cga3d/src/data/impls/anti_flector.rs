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
//   Median:         1       2       0     N/A
//  Average:         6       9       0     N/A
//  Maximum:       105     127       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         4       7       0       0
//  Average:        14      18       0       0
//  Maximum:       217     247       0       0
impl std::ops::Add<AntiCircleRotor> for AntiFlector {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group2().xyz().with_w(other[e45]),
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<AntiDipoleInversion> for AntiFlector {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        3        0        0      N/A
    //  no simd        9        0        0        0
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0(),
            // e415, e425, e435, e321
            other.group1().xyz().with_w(other[e321] + self[e321]),
            // e235, e315, e125, e4
            other.group2() + self.group0().xyz().with_w(0.0),
            // e1, e2, e3, e5
            other.group3() + self.group1(),
        )
    }
}
impl std::ops::Add<AntiDualNum> for AntiFlector {
    type Output = MultiVector;
    fn add(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(other[e3215]),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<AntiFlatPoint> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        1        0        0      N/A
    // no simd        4        0        0        0
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        AntiFlector::from_groups(/* e235, e315, e125, e321 */ other.group0() + self.group0(), /* e1, e2, e3, e5 */ self.group1())
    }
}
impl std::ops::AddAssign<AntiFlatPoint> for AntiFlector {
    fn add_assign(&mut self, other: AntiFlatPoint) {
        *self = AntiFlector::from_groups(/* e235, e315, e125, e321 */ other.group0() + self.group0(), /* e1, e2, e3, e5 */ self.group1());
    }
}
impl std::ops::Add<AntiFlector> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        2        0        0      N/A
    // no simd        8        0        0        0
    fn add(self, other: AntiFlector) -> Self::Output {
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() + self.group0(),
            // e1, e2, e3, e5
            other.group1() + self.group1(),
        )
    }
}
impl std::ops::AddAssign<AntiFlector> for AntiFlector {
    fn add_assign(&mut self, other: AntiFlector) {
        *self = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() + self.group0(),
            // e1, e2, e3, e5
            other.group1() + self.group1(),
        );
    }
}
impl std::ops::Add<AntiLine> for AntiFlector {
    type Output = MultiVector;
    fn add(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group1().with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<AntiMotor> for AntiFlector {
    type Output = MultiVector;
    fn add(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group1().xyz().with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(other[e3215]),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<AntiPlane> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        1        0        0      N/A
    // no simd        4        0        0        0
    fn add(self, other: AntiPlane) -> Self::Output {
        AntiFlector::from_groups(/* e235, e315, e125, e321 */ self.group0(), /* e1, e2, e3, e5 */ self.group1() + other.group0())
    }
}
impl std::ops::AddAssign<AntiPlane> for AntiFlector {
    fn add_assign(&mut self, other: AntiPlane) {
        *self = AntiFlector::from_groups(/* e235, e315, e125, e321 */ self.group0(), /* e1, e2, e3, e5 */ self.group1() + other.group0());
    }
}
impl std::ops::Add<AntiScalar> for AntiFlector {
    type Output = VersorEven;
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(other[e12345]),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e235, e315, e125, e5
            self.group0().xyz().with_w(self[e5]),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(0.0),
        )
    }
}
impl std::ops::Add<Circle> for AntiFlector {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd3        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        4        0        0        0
    fn add(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0(),
            // e415, e425, e435, e321
            other.group1().xyz().with_w(self[e321] + other[e321]),
            // e235, e315, e125, e4
            (other.group2() + self.group0().xyz()).with_w(0.0),
            // e1, e2, e3, e5
            self.group1(),
        )
    }
}
impl std::ops::Add<CircleRotor> for AntiFlector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd3        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        4        0        0        0
    fn add(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0().with_w(other[e12345]),
            // e415, e425, e435, e321
            other.group1().xyz().with_w(self[e321] + other[e321]),
            // e235, e315, e125, e5
            (self.group0().xyz() + other.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(0.0),
        )
    }
}
impl std::ops::Add<Dipole> for AntiFlector {
    type Output = MultiVector;
    fn add(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group2().with_w(other[e45]),
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<DipoleInversion> for AntiFlector {
    type Output = MultiVector;
    fn add(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group2().xyz().with_w(other[e45]),
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            other.group3(),
            // e1234
            other[e1234],
        )
    }
}
impl std::ops::Add<DualNum> for AntiFlector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        1        0        0        0
    fn add(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(other[e12345]),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e235, e315, e125, e5
            self.group0().xyz().with_w(other[e5] + self[e5]),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(0.0),
        )
    }
}
impl std::ops::Add<FlatPoint> for AntiFlector {
    type Output = MultiVector;
    fn add(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<Flector> for AntiFlector {
    type Output = MultiVector;
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            other.group1(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<Line> for AntiFlector {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        1        0        0      N/A
    // no simd        3        0        0        0
    fn add(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0().with_w(self[e321]),
            // e235, e315, e125, e4
            (other.group1() + self.group0().xyz()).with_w(0.0),
            // e1, e2, e3, e5
            self.group1(),
        )
    }
}
impl std::ops::Add<Motor> for AntiFlector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        1        0        0      N/A
    // no simd        4        0        0        0
    fn add(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(other[e12345]),
            // e415, e425, e435, e321
            other.group0().xyz().with_w(self[e321]),
            // e235, e315, e125, e5
            other.group1() + self.group0().xyz().with_w(self[e5]),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(0.0),
        )
    }
}
impl std::ops::Add<MultiVector> for AntiFlector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        0        0        0
    //    simd3        2        0        0      N/A
    // Totals...
    // yes simd        4        0        0      N/A
    //  no simd        8        0        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            other.group0(),
            // e1, e2, e3, e4
            (self.group1().xyz() + other.group1().xyz()).with_w(other[e4]),
            // e5
            self[e5] + other[e5],
            // e15, e25, e35, e45
            other.group3(),
            // e41, e42, e43
            other.group4(),
            // e23, e31, e12
            other.group5(),
            // e415, e425, e435, e321
            other.group6().xyz().with_w(self[e321] + other[e321]),
            // e423, e431, e412
            other.group7(),
            // e235, e315, e125
            other.group8() + self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            other.group9(),
            // e1234
            other[e1234],
        )
    }
}
impl std::ops::Add<Plane> for AntiFlector {
    type Output = MultiVector;
    fn add(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            other.group0(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<RoundPoint> for AntiFlector {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        1        0        0      N/A
    // no simd        4        0        0        0
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e235, e315, e125, e4
            self.group0().xyz().with_w(other[e4]),
            // e1, e2, e3, e5
            self.group1() + other.group0().xyz().with_w(other[e5]),
        )
    }
}
impl std::ops::Add<Scalar> for AntiFlector {
    type Output = MultiVector;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<Sphere> for AntiFlector {
    type Output = MultiVector;
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            other.group0(),
            // e1234
            other[e1234],
        )
    }
}
impl std::ops::Add<VersorEven> for AntiFlector {
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
            other.group0(),
            // e415, e425, e435, e321
            other.group1().xyz().with_w(self[e321] + other[e321]),
            // e235, e315, e125, e5
            other.group2() + self.group0().xyz().with_w(self[e5]),
            // e1, e2, e3, e4
            (self.group1().xyz() + other.group3().xyz()).with_w(other[e4]),
        )
    }
}
impl std::ops::Add<VersorOdd> for AntiFlector {
    type Output = MultiVector;
    fn add(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group2().xyz().with_w(other[e45]),
            // e41, e42, e43
            other.group0().xyz(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            other.group3(),
            // e1234
            other[e1234],
        )
    }
}
impl std::ops::BitXor<AntiCircleRotor> for AntiFlector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       12        0        0
    //    simd3        3        6        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd        9       19        0      N/A
    //  no simd       18       34        0        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for AntiFlector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       11        0        0
    //    simd2        1        3        0      N/A
    //    simd3        2        4        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        9       20        0      N/A
    //  no simd       20       37        0        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<AntiDualNum> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn bitxor(self, other: AntiDualNum) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<AntiDualNum> for AntiFlector {
    fn bitxor_assign(&mut self, other: AntiDualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatPoint> for AntiFlector {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn bitxor(self, other: AntiFlatPoint) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<AntiFlector> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd2        1        2        0      N/A
    //    simd3        0        1        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        7       10        0      N/A
    //  no simd       14       17        0        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<AntiLine> for AntiFlector {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        3        0      N/A
    // no simd        6        9        0        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<AntiMotor> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd2        1        2        0      N/A
    //    simd3        1        1        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        5        7        0      N/A
    //  no simd       14       17        0        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<AntiMotor> for AntiFlector {
    fn bitxor_assign(&mut self, other: AntiMotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd2        1        2        0      N/A
    //    simd3        0        1        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd       11       13        0        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Circle> for AntiFlector {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd2        1        2        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        6        8        0      N/A
    //  no simd       13       13        0        0
    fn bitxor(self, other: Circle) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<CircleRotor> for AntiFlector {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd2        1        2        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        6        8        0      N/A
    //  no simd       13       13        0        0
    fn bitxor(self, other: CircleRotor) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Dipole> for AntiFlector {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        4        7        0      N/A
    // no simd       12       21        0        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<DipoleInversion> for AntiFlector {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2        4        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        7       13        0      N/A
    //  no simd       17       28        0        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<DualNum> for AntiFlector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        8        0        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<FlatPoint> for AntiFlector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd2        1        2        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        7       11        0        0
    fn bitxor(self, other: FlatPoint) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Flector> for AntiFlector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd2        1        2        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        6       10        0      N/A
    //  no simd       10       16        0        0
    fn bitxor(self, other: Flector) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Line> for AntiFlector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        1        2        0      N/A
    // no simd        3        6        0        0
    fn bitxor(self, other: Line) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Motor> for AntiFlector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        1        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        6       13        0        0
    fn bitxor(self, other: Motor) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<MultiVector> for AntiFlector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       20       35        0        0
    //    simd2        0        1        0      N/A
    //    simd3        5       10        0      N/A
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd       28       50        0      N/A
    //  no simd       47       83        0        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Plane> for AntiFlector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn bitxor(self, other: Plane) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<RoundPoint> for AntiFlector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       11        0        0
    //    simd2        1        2        0      N/A
    //    simd3        1        5        0      N/A
    // Totals...
    // yes simd        6       18        0      N/A
    //  no simd        9       30        0        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Scalar> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<Scalar> for AntiFlector {
    fn bitxor_assign(&mut self, other: Scalar) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Sphere> for AntiFlector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn bitxor(self, other: Sphere) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<VersorEven> for AntiFlector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        9        0        0
    //    simd2        1        3        0      N/A
    //    simd3        1        2        0      N/A
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd        8       18        0      N/A
    //  no simd       20       37        0        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<VersorOdd> for AntiFlector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       15        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2        3        0      N/A
    //    simd4        3        3        0      N/A
    // Totals...
    // yes simd       12       22        0      N/A
    //  no simd       25       38        0        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        self.wedge(other)
    }
}

impl From<AntiFlatPoint> for AntiFlector {
    fn from(from_anti_flat_point: AntiFlatPoint) -> Self {
        AntiFlector::from_groups(/* e235, e315, e125, e321 */ from_anti_flat_point.group0(), /* e1, e2, e3, e5 */ Simd32x4::from(0.0))
    }
}

impl From<AntiPlane> for AntiFlector {
    fn from(from_anti_plane: AntiPlane) -> Self {
        AntiFlector::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(0.0), /* e1, e2, e3, e5 */ from_anti_plane.group0())
    }
}
impl std::ops::Mul<AntiCircleRotor> for AntiFlector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       15        0        0
    //    simd2        5        7        0      N/A
    //    simd3        8       10        0      N/A
    //    simd4        6        5        0      N/A
    // Totals...
    // yes simd       30       37        0      N/A
    //  no simd       69       79        0        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiDipoleInversion> for AntiFlector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       18       24        0        0
    //    simd2       10       12        0      N/A
    //    simd3        9       11        0      N/A
    //    simd4        7        7        0      N/A
    // Totals...
    // yes simd       44       54        0      N/A
    //  no simd       93      109        0        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiDualNum> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        4       12        0        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<AntiDualNum> for AntiFlector {
    fn mul_assign(&mut self, other: AntiDualNum) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd2        2        3        0      N/A
    //    simd3        1        1        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd        6        9        0      N/A
    //  no simd       13       17        0        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiFlector> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       10        0        0
    //    simd2        4        6        0      N/A
    //    simd4        7        6        0      N/A
    // Totals...
    // yes simd       18       22        0      N/A
    //  no simd       43       46        0        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiLine> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        5        0        0
    //    simd3        7        9        0      N/A
    // Totals...
    // yes simd       11       14        0      N/A
    //  no simd       25       32        0        0
    fn mul(self, other: AntiLine) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<AntiLine> for AntiFlector {
    fn mul_assign(&mut self, other: AntiLine) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7        9        0        0
    //    simd2        4        5        0      N/A
    //    simd3        2        3        0      N/A
    //    simd4        5        4        0      N/A
    // Totals...
    // yes simd       18       21        0      N/A
    //  no simd       41       44        0        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<AntiMotor> for AntiFlector {
    fn mul_assign(&mut self, other: AntiMotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd2        0        2        0      N/A
    //    simd3        2        2        0      N/A
    //    simd4        5        3        0      N/A
    // Totals...
    // yes simd        7       11        0      N/A
    //  no simd       26       26        0        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiScalar> for AntiFlector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Circle> for AntiFlector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       13        0        0
    //    simd2        2        3        0      N/A
    //    simd3        8       11        0      N/A
    //    simd4        6        6        0      N/A
    // Totals...
    // yes simd       24       33        0      N/A
    //  no simd       60       76        0        0
    fn mul(self, other: Circle) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<CircleRotor> for AntiFlector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       17        0        0
    //    simd2        6        8        0      N/A
    //    simd3        7        9        0      N/A
    //    simd4        5        5        0      N/A
    // Totals...
    // yes simd       29       39        0      N/A
    //  no simd       64       80        0        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Dipole> for AntiFlector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       10        0        0
    //    simd2        1        3        0      N/A
    //    simd3       10       12        0      N/A
    //    simd4        6        5        0      N/A
    // Totals...
    // yes simd       23       30        0      N/A
    //  no simd       62       72        0        0
    fn mul(self, other: Dipole) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<DipoleInversion> for AntiFlector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       17        0        0
    //    simd2        4        7        0      N/A
    //    simd3       11       12        0      N/A
    //    simd4       12       11        0      N/A
    // Totals...
    // yes simd       38       47        0      N/A
    //  no simd      100      111        0        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<DualNum> for AntiFlector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        4       12        0        0
    fn mul(self, other: DualNum) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<FlatPoint> for AntiFlector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd2        0        1        0      N/A
    //    simd3        1        1        0      N/A
    //    simd4        3        3        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd       15       20        0        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Flector> for AntiFlector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        8        0        0
    //    simd2        4        6        0      N/A
    //    simd3        2        2        0      N/A
    //    simd4        5        4        0      N/A
    // Totals...
    // yes simd       16       20        0      N/A
    //  no simd       39       42        0        0
    fn mul(self, other: Flector) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Line> for AntiFlector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        6        8        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd       10       13        0      N/A
    //  no simd       25       32        0        0
    fn mul(self, other: Line) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Motor> for AntiFlector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd2        4        5        0      N/A
    //    simd3        2        2        0      N/A
    //    simd4        6        5        0      N/A
    // Totals...
    // yes simd       18       20        0      N/A
    //  no simd       44       44        0        0
    fn mul(self, other: Motor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<MultiVector> for AntiFlector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       51       67        0        0
    //    simd2        5        9        0      N/A
    //    simd3       40       42        0      N/A
    //    simd4        9        9        0      N/A
    // Totals...
    // yes simd      105      127        0      N/A
    //  no simd      217      247        0        0
    fn mul(self, other: MultiVector) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Plane> for AntiFlector {
    type Output = Motor;
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
impl std::ops::Mul<RoundPoint> for AntiFlector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       24        0        0
    //    simd3        4        8        0      N/A
    // Totals...
    // yes simd       16       32        0      N/A
    //  no simd       24       48        0        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Scalar> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn mul(self, other: Scalar) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Scalar> for AntiFlector {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for AntiFlector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       14        0        0
    //    simd2        0        1        0      N/A
    //    simd3        3        4        0      N/A
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd       12       23        0      N/A
    //  no simd       27       44        0        0
    fn mul(self, other: Sphere) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<VersorEven> for AntiFlector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       23       31        0        0
    //    simd2       14       18        0      N/A
    //    simd3        5        5        0      N/A
    //    simd4       10        9        0      N/A
    // Totals...
    // yes simd       52       63        0      N/A
    //  no simd      106      118        0        0
    fn mul(self, other: VersorEven) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<VersorOdd> for AntiFlector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       16        0        0
    //    simd2        5        8        0      N/A
    //    simd3        4        5        0      N/A
    //    simd4       21       19        0      N/A
    // Totals...
    // yes simd       40       48        0      N/A
    //  no simd      116      123        0        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Neg for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn neg(self) -> Self::Output {
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group0() * Simd32x4::from(-1.0),
            // e1, e2, e3, e5
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Not for AntiFlector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn not(self) -> Self::Output {
        self.right_dual()
    }
}
impl std::ops::Sub<AntiCircleRotor> for AntiFlector {
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
            self.group1().xyz().with_w(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            (other.group2().xyz() * Simd32x3::from(-1.0)).with_w(other[e45] * -1.0),
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<AntiDipoleInversion> for AntiFlector {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        1        0        0
    //    simd2        1        0        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        4        3        0      N/A
    //  no simd        8        7        0        0
    fn sub(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(self[e321] - other[e321]),
            // e235, e315, e125, e4
            (self.group0().xy() - other.group2().xy()).with_zw(self[e125] - other[e125], other[e4] * -1.0),
            // e1, e2, e3, e5
            self.group1() - other.group3(),
        )
    }
}
impl std::ops::Sub<AntiDualNum> for AntiFlector {
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
            self.group1().xyz().with_w(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<AntiFlatPoint> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        1        0        0      N/A
    // no simd        4        0        0        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        AntiFlector::from_groups(/* e235, e315, e125, e321 */ self.group0() - other.group0(), /* e1, e2, e3, e5 */ self.group1())
    }
}
impl std::ops::SubAssign<AntiFlatPoint> for AntiFlector {
    fn sub_assign(&mut self, other: AntiFlatPoint) {
        *self = AntiFlector::from_groups(/* e235, e315, e125, e321 */ self.group0() - other.group0(), /* e1, e2, e3, e5 */ self.group1());
    }
}
impl std::ops::Sub<AntiFlector> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        2        0        0      N/A
    // no simd        8        0        0        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group0() - other.group0(),
            // e1, e2, e3, e5
            self.group1() - other.group1(),
        )
    }
}
impl std::ops::SubAssign<AntiFlector> for AntiFlector {
    fn sub_assign(&mut self, other: AntiFlector) {
        *self = AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group0() - other.group0(),
            // e1, e2, e3, e5
            self.group1() - other.group1(),
        );
    }
}
impl std::ops::Sub<AntiLine> for AntiFlector {
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
            self.group1().xyz().with_w(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            (other.group1() * Simd32x3::from(-1.0)).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<AntiMotor> for AntiFlector {
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
            self.group1().xyz().with_w(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<AntiPlane> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        1        0        0      N/A
    // no simd        4        0        0        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        AntiFlector::from_groups(/* e235, e315, e125, e321 */ self.group0(), /* e1, e2, e3, e5 */ self.group1() - other.group0())
    }
}
impl std::ops::SubAssign<AntiPlane> for AntiFlector {
    fn sub_assign(&mut self, other: AntiPlane) {
        *self = AntiFlector::from_groups(/* e235, e315, e125, e321 */ self.group0(), /* e1, e2, e3, e5 */ self.group1() - other.group0());
    }
}
impl std::ops::Sub<AntiScalar> for AntiFlector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(other[e12345] * -1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e235, e315, e125, e5
            self.group0().xyz().with_w(self[e5]),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(0.0),
        )
    }
}
impl std::ops::Sub<Circle> for AntiFlector {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd3        1        2        0      N/A
    // Totals...
    // yes simd        2        2        0      N/A
    //  no simd        4        6        0        0
    fn sub(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(self[e321] - other[e321]),
            // e235, e315, e125, e4
            (self.group0().xyz() - other.group2()).with_w(0.0),
            // e1, e2, e3, e5
            self.group1(),
        )
    }
}
impl std::ops::Sub<CircleRotor> for AntiFlector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        1        0        0
    //    simd3        1        2        0      N/A
    // Totals...
    // yes simd        2        3        0      N/A
    //  no simd        4        7        0        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() * Simd32x3::from(-1.0)).with_w(other[e12345] * -1.0),
            // e415, e425, e435, e321
            (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(self[e321] - other[e321]),
            // e235, e315, e125, e5
            (self.group0().xyz() - other.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(0.0),
        )
    }
}
impl std::ops::Sub<Dipole> for AntiFlector {
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
            self.group1().xyz().with_w(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            (other.group2() * Simd32x3::from(-1.0)).with_w(other[e45] * -1.0),
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<DipoleInversion> for AntiFlector {
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
            self.group1().xyz().with_w(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            (other.group2().xyz() * Simd32x3::from(-1.0)).with_w(other[e45] * -1.0),
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from(-1.0),
            // e1234
            other[e1234] * -1.0,
        )
    }
}
impl std::ops::Sub<DualNum> for AntiFlector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        1        1        0        0
    fn sub(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(other[e12345] * -1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e235, e315, e125, e5
            self.group0().xyz().with_w(self[e5] - other[e5]),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(0.0),
        )
    }
}
impl std::ops::Sub<FlatPoint> for AntiFlector {
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
            self.group1().xyz().with_w(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from(-1.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<Flector> for AntiFlector {
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
            self.group1().xyz().with_w(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from(-1.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from(-1.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<Line> for AntiFlector {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        1        1        0      N/A
    // no simd        3        3        0        0
    fn sub(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (other.group0() * Simd32x3::from(-1.0)).with_w(self[e321]),
            // e235, e315, e125, e4
            (self.group0().xyz() - other.group1()).with_w(0.0),
            // e1, e2, e3, e5
            self.group1(),
        )
    }
}
impl std::ops::Sub<Motor> for AntiFlector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        1        2        0      N/A
    //  no simd        4        4        0        0
    fn sub(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(other[e12345] * -1.0),
            // e415, e425, e435, e321
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e321]),
            // e235, e315, e125, e5
            self.group0().xyz().with_w(self[e5]) - other.group1(),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(0.0),
        )
    }
}
impl std::ops::Sub<MultiVector> for AntiFlector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        2        0        0
    //    simd2        1        1        0      N/A
    //    simd3        1        4        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        5        9        0      N/A
    //  no simd        8       24        0        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            other.group0() * Simd32x2::from(-1.0),
            // e1, e2, e3, e4
            (self.group1().xy() - other.group1().xy()).with_zw(self[e3] - other[e3], other[e4] * -1.0),
            // e5
            self[e5] - other[e5],
            // e15, e25, e35, e45
            other.group3() * Simd32x4::from(-1.0),
            // e41, e42, e43
            other.group4() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group5() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            (other.group6().xyz() * Simd32x3::from(-1.0)).with_w(self[e321] - other[e321]),
            // e423, e431, e412
            other.group7() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group0().xyz() - other.group8(),
            // e4235, e4315, e4125, e3215
            other.group9() * Simd32x4::from(-1.0),
            // e1234
            other[e1234] * -1.0,
        )
    }
}
impl std::ops::Sub<Plane> for AntiFlector {
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
            self.group1().xyz().with_w(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            other.group0() * Simd32x4::from(-1.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<RoundPoint> for AntiFlector {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        1        2        0      N/A
    //  no simd        4        2        0        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e235, e315, e125, e4
            self.group0().xyz().with_w(other[e4] * -1.0),
            // e1, e2, e3, e5
            self.group1() + -other.group0().xyz().with_w(other[e5] * -1.0),
        )
    }
}
impl std::ops::Sub<Scalar> for AntiFlector {
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
            self.group1().xyz().with_w(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<Sphere> for AntiFlector {
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
            self.group1().xyz().with_w(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            other.group0() * Simd32x4::from(-1.0),
            // e1234
            other[e1234] * -1.0,
        )
    }
}
impl std::ops::Sub<VersorEven> for AntiFlector {
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
            other.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435, e321
            (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(self[e321] - other[e321]),
            // e235, e315, e125, e5
            self.group0().xyz().with_w(self[e5]) - other.group2(),
            // e1, e2, e3, e4
            (self.group1().xy() - other.group3().xy()).with_zw(self[e3] - other[e3], other[e4] * -1.0),
        )
    }
}
impl std::ops::Sub<VersorOdd> for AntiFlector {
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
            self.group1().xyz().with_w(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            (other.group2().xyz() * Simd32x3::from(-1.0)).with_w(other[e45] * -1.0),
            // e41, e42, e43
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from(-1.0),
            // e1234
            other[e1234] * -1.0,
        )
    }
}

impl TryFrom<AntiDipoleInversion> for AntiFlector {
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
        let el = anti_dipole_inversion[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
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
        if fail {
            let mut error = "Elements from AntiDipoleInversion do not fit into AntiFlector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(AntiFlector::from_groups(
            // e235, e315, e125, e321
            anti_dipole_inversion.group2().xyz().with_w(anti_dipole_inversion[e321]),
            // e1, e2, e3, e5
            anti_dipole_inversion.group3(),
        ))
    }
}

impl TryFrom<Circle> for AntiFlector {
    type Error = String;
    fn try_from(circle: Circle) -> Result<Self, Self::Error> {
        use crate::elements::*;
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
        let el = circle[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Circle do not fit into AntiFlector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(AntiFlector::from_groups(
            // e235, e315, e125, e321
            circle.group2().with_w(circle[e321]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<CircleRotor> for AntiFlector {
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
        let el = circle_rotor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotor do not fit into AntiFlector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(AntiFlector::from_groups(
            // e235, e315, e125, e321
            circle_rotor.group2().xyz().with_w(circle_rotor[e321]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<DualNum> for AntiFlector {
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
            let mut error = "Elements from DualNum do not fit into AntiFlector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(dual_num[e5]),
        ))
    }
}

impl TryFrom<Line> for AntiFlector {
    type Error = String;
    fn try_from(line: Line) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = line[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = line[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = line[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Line do not fit into AntiFlector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(AntiFlector::from_groups(
            // e235, e315, e125, e321
            line.group1().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<Motor> for AntiFlector {
    type Error = String;
    fn try_from(motor: Motor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = motor[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Motor do not fit into AntiFlector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(AntiFlector::from_groups(
            // e235, e315, e125, e321
            motor.group1().xyz().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(motor[e5]),
        ))
    }
}

impl TryFrom<MultiVector> for AntiFlector {
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
            let mut error = "Elements from MultiVector do not fit into AntiFlector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(AntiFlector::from_groups(
            // e235, e315, e125, e321
            multi_vector.group8().with_w(multi_vector[e321]),
            // e1, e2, e3, e5
            multi_vector.group1().xyz().with_w(multi_vector[e5]),
        ))
    }
}

impl TryFrom<RoundPoint> for AntiFlector {
    type Error = String;
    fn try_from(round_point: RoundPoint) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = round_point[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from RoundPoint do not fit into AntiFlector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            round_point.group0().xyz().with_w(round_point[e5]),
        ))
    }
}

impl TryFrom<VersorEven> for AntiFlector {
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
        let el = versor_even[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
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
            let mut error = "Elements from VersorEven do not fit into AntiFlector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(AntiFlector::from_groups(
            // e235, e315, e125, e321
            versor_even.group2().xyz().with_w(versor_even[e321]),
            // e1, e2, e3, e5
            versor_even.group3().xyz().with_w(versor_even[e5]),
        ))
    }
}
