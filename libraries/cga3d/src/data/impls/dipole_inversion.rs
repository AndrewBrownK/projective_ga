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
// Total Implementations: 110
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         2       3       0     N/A
//  Average:        12      15       0     N/A
//  Maximum:       209     233       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         6       6       0       0
//  Average:        26      31       0       0
//  Maximum:       426     456       0       0
impl std::ops::Add<AntiCircleRotor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        2        0        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        3        0        0      N/A
    //  no simd       10        0        0        0
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (other.group0() + self.group0()).with_w(other[scalar]),
            // e23, e31, e12, e45
            other.group1() + self.group1(),
            // e15, e25, e35, e1234
            (other.group2().xyz() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        )
    }
}
impl std::ops::Add<AntiDipoleInversion> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(other[e4]),
            // e5
            other[e5],
            // e15, e25, e35, e45
            self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            self.group0(),
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
impl std::ops::Add<AntiDualNum> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        1        0        0        0
    fn add(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(other[scalar]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3().xyz().with_w(other[e3215] + self[e3215]),
        )
    }
}
impl std::ops::Add<AntiFlatPoint> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            self.group0(),
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
impl std::ops::Add<AntiFlector> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group1().xyz().with_w(0.0),
            // e5
            other[e5],
            // e15, e25, e35, e45
            self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            self.group0(),
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
impl std::ops::Add<AntiLine> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        2        0        0      N/A
    // no simd        8        0        0        0
    fn add(self, other: AntiLine) -> Self::Output {
        DipoleInversion::from_groups(
            // e41, e42, e43
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
impl std::ops::AddAssign<AntiLine> for DipoleInversion {
    fn add_assign(&mut self, other: AntiLine) {
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
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
impl std::ops::Add<AntiMotor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd3        2        0        0      N/A
    // Totals...
    // yes simd        3        0        0      N/A
    //  no simd        7        0        0        0
    fn add(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(other[scalar]),
            // e23, e31, e12, e45
            (other.group0().xyz() + self.group1().xyz()).with_w(self[e45]),
            // e15, e25, e35, e1234
            (other.group1().xyz() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3().xyz().with_w(other[e3215] + self[e3215]),
        )
    }
}
impl std::ops::Add<AntiPlane> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group0().xyz().with_w(0.0),
            // e5
            other[e5],
            // e15, e25, e35, e45
            self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            self.group0(),
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
impl std::ops::Add<AntiScalar> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            self.group0(),
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
impl std::ops::Add<Circle> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            self.group0(),
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
impl std::ops::Add<CircleRotor> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            self.group0(),
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
impl std::ops::Add<Dipole> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        1        0        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        3        0        0      N/A
    //  no simd       11        0        0        0
    fn add(self, other: Dipole) -> Self::Output {
        DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0() + self.group0(),
            // e23, e31, e12, e45
            other.group1() + self.group1(),
            // e15, e25, e35, e1234
            self.group2() + other.group2().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        )
    }
}
impl std::ops::AddAssign<Dipole> for DipoleInversion {
    fn add_assign(&mut self, other: Dipole) {
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0() + self.group0(),
            // e23, e31, e12, e45
            other.group1() + self.group1(),
            // e15, e25, e35, e1234
            self.group2() + other.group2().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<DipoleInversion> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        1        0        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        4        0        0      N/A
    //  no simd       15        0        0        0
    fn add(self, other: DipoleInversion) -> Self::Output {
        DipoleInversion::from_groups(
            // e41, e42, e43
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
impl std::ops::AddAssign<DipoleInversion> for DipoleInversion {
    fn add_assign(&mut self, other: DipoleInversion) {
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
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
impl std::ops::Add<DualNum> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5],
            // e15, e25, e35, e45
            self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            self.group0(),
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
impl std::ops::Add<FlatPoint> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        5        0        0        0
    fn add(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e45] + other[e45]),
            // e15, e25, e35, e1234
            self.group2() + other.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        )
    }
}
impl std::ops::AddAssign<FlatPoint> for DipoleInversion {
    fn add_assign(&mut self, other: FlatPoint) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e45] + other[e45]),
            // e15, e25, e35, e1234
            self.group2() + other.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<Flector> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        3        0        0      N/A
    //  no simd        9        0        0        0
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e45] + other[e45]),
            // e15, e25, e35, e1234
            self.group2() + other.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3() + other.group1(),
        )
    }
}
impl std::ops::AddAssign<Flector> for DipoleInversion {
    fn add_assign(&mut self, other: Flector) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e45] + other[e45]),
            // e15, e25, e35, e1234
            self.group2() + other.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3() + other.group1(),
        );
    }
}
impl std::ops::Add<Line> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: Line) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            self.group0(),
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
impl std::ops::Add<Motor> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5],
            // e15, e25, e35, e45
            self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            self.group0(),
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
impl std::ops::Add<MultiVector> for DipoleInversion {
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
            other.group1(),
            // e5
            other[e5],
            // e15, e25, e35, e45
            other.group3() + self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            self.group0() + other.group4(),
            // e23, e31, e12
            other.group5() + self.group1().xyz(),
            // e415, e425, e435, e321
            other.group6(),
            // e423, e431, e412
            other.group7(),
            // e235, e315, e125
            other.group8(),
            // e4235, e4315, e4125, e3215
            self.group3() + other.group9(),
            // e1234
            self[e1234] + other[e1234],
        )
    }
}
impl std::ops::Add<Plane> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        1        0        0      N/A
    // no simd        4        0        0        0
    fn add(self, other: Plane) -> Self::Output {
        DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() + other.group0(),
        )
    }
}
impl std::ops::AddAssign<Plane> for DipoleInversion {
    fn add_assign(&mut self, other: Plane) {
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() + other.group0(),
        );
    }
}
impl std::ops::Add<RoundPoint> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group0(),
            // e5
            other[e5],
            // e15, e25, e35, e45
            self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            self.group0(),
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
impl std::ops::Add<Scalar> for DipoleInversion {
    type Output = VersorOdd;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(other[scalar]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        )
    }
}
impl std::ops::Add<Sphere> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        5        0        0        0
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(self[e1234] + other[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3() + other.group0(),
        )
    }
}
impl std::ops::AddAssign<Sphere> for DipoleInversion {
    fn add_assign(&mut self, other: Sphere) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(self[e1234] + other[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3() + other.group0(),
        );
    }
}
impl std::ops::Add<VersorEven> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            other.group3(),
            // e5
            other[e5],
            // e15, e25, e35, e45
            self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            self.group0(),
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
impl std::ops::Add<VersorOdd> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        4        0        0      N/A
    // no simd       16        0        0        0
    fn add(self, other: VersorOdd) -> Self::Output {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() + self.group0().with_w(0.0),
            // e23, e31, e12, e45
            self.group1() + other.group1(),
            // e15, e25, e35, e1234
            self.group2() + other.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() + other.group3(),
        )
    }
}
impl std::ops::BitXor<AntiCircleRotor> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd3        6        9        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       12       18        0      N/A
    //  no simd       24       39        0        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        self.wedge(other)
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
    //           add/sub      mul      div      pow
    //      f32       11       13        0        0
    //    simd2        1        2        0      N/A
    //    simd3        4        6        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd       17       22        0      N/A
    //  no simd       29       39        0        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<AntiDualNum> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       15        0        0
    fn bitxor(self, other: AntiDualNum) -> Self::Output {
        self.wedge(other)
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
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn bitxor(self, other: AntiFlatPoint) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<AntiFlector> for DipoleInversion {
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
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<AntiLine> for DipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        2        3        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd        8       12        0        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<AntiMotor> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        3        6        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        6       12        0      N/A
    //  no simd       12       27        0        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        self.wedge(other)
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
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2        4        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        5        9        0      N/A
    //  no simd       18       24        0        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Circle> for DipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        9       10        0        0
    fn bitxor(self, other: Circle) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<CircleRotor> for DipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        9       10        0        0
    fn bitxor(self, other: CircleRotor) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Dipole> for DipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        5        6        0      N/A
    // Totals...
    // yes simd       10       12        0      N/A
    //  no simd       20       24        0        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<DipoleInversion> for DipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        5        6        0      N/A
    // Totals...
    // yes simd       10       12        0      N/A
    //  no simd       20       24        0        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<DualNum> for DipoleInversion {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       11        0        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<FlatPoint> for DipoleInversion {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        3        0      N/A
    // no simd        6        9        0        0
    fn bitxor(self, other: FlatPoint) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Flector> for DipoleInversion {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        3        0      N/A
    // no simd        6        9        0        0
    fn bitxor(self, other: Flector) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Line> for DipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        6        0        0
    fn bitxor(self, other: Line) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Motor> for DipoleInversion {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        7        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        6        9        0      N/A
    //  no simd        6       13        0        0
    fn bitxor(self, other: Motor) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<MultiVector> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       23       28        0        0
    //    simd3       10       15        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd       34       45        0      N/A
    //  no simd       57       81        0        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<RoundPoint> for DipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4        6        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        7       11        0      N/A
    //  no simd       24       30        0        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Scalar> for DipoleInversion {
    type Output = DipoleInversion;
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
impl std::ops::BitXorAssign<Scalar> for DipoleInversion {
    fn bitxor_assign(&mut self, other: Scalar) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for DipoleInversion {
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
    fn bitxor(self, other: VersorEven) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<VersorOdd> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       10        0        0
    //    simd2        1        2        0      N/A
    //    simd3        5        7        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       13       20        0      N/A
    //  no simd       24       39        0        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<VersorOdd> for DipoleInversion {
    fn bitxor_assign(&mut self, other: VersorOdd) {
        *self = self.wedge(other);
    }
}

impl From<AntiLine> for DipoleInversion {
    fn from(from_anti_line: AntiLine) -> Self {
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            from_anti_line.group0().with_w(0.0),
            // e15, e25, e35, e1234
            from_anti_line.group1().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<Dipole> for DipoleInversion {
    fn from(from_dipole: Dipole) -> Self {
        DipoleInversion::from_groups(
            // e41, e42, e43
            from_dipole.group0(),
            // e23, e31, e12, e45
            from_dipole.group1(),
            // e15, e25, e35, e1234
            from_dipole.group2().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<FlatPoint> for DipoleInversion {
    fn from(from_flat_point: FlatPoint) -> Self {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_flat_point[e45]),
            // e15, e25, e35, e1234
            from_flat_point.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<Flector> for DipoleInversion {
    fn from(from_flector: Flector) -> Self {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_flector[e45]),
            // e15, e25, e35, e1234
            from_flector.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            from_flector.group1(),
        )
    }
}

impl From<Plane> for DipoleInversion {
    fn from(from_plane: Plane) -> Self {
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            from_plane.group0(),
        )
    }
}

impl From<Sphere> for DipoleInversion {
    fn from(from_sphere: Sphere) -> Self {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_sphere[e1234]),
            // e4235, e4315, e4125, e3215
            from_sphere.group0(),
        )
    }
}
impl std::ops::Mul<AntiCircleRotor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       20        0        0
    //    simd2        7       10        0      N/A
    //    simd3       25       26        0      N/A
    //    simd4        5        5        0      N/A
    // Totals...
    // yes simd       49       61        0      N/A
    //  no simd      121      138        0        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiDipoleInversion> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       41       48        0        0
    //    simd2       15       18        0      N/A
    //    simd3       31       32        0      N/A
    //    simd4        6        6        0      N/A
    // Totals...
    // yes simd       93      104        0      N/A
    //  no simd      188      204        0        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiDualNum> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        4        8        0      N/A
    // Totals...
    // yes simd        4       12        0      N/A
    //  no simd       12       28        0        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiFlatPoint> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       10        0        0
    //    simd2        3        4        0      N/A
    //    simd3        7       11        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd       18       26        0      N/A
    //  no simd       41       55        0        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiFlector> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       14       21        0        0
    //    simd2        9       12        0      N/A
    //    simd3       15       16        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd       40       51        0      N/A
    //  no simd       85      101        0        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiLine> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3       18       22        0      N/A
    // Totals...
    // yes simd       23       28        0      N/A
    //  no simd       59       72        0        0
    fn mul(self, other: AntiLine) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiMotor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       15        0        0
    //    simd2        8       11        0      N/A
    //    simd3       10       11        0      N/A
    //    simd4        9        8        0      N/A
    // Totals...
    // yes simd       36       45        0      N/A
    //  no simd       91      102        0        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiPlane> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        7        0        0
    //    simd2        3        6        0      N/A
    //    simd3        3        4        0      N/A
    //    simd4        8        5        0      N/A
    // Totals...
    // yes simd       17       22        0      N/A
    //  no simd       50       51        0        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiScalar> for DipoleInversion {
    type Output = AntiDipoleInversion;
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
impl std::ops::Mul<Circle> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       20        0        0
    //    simd2        4        7        0      N/A
    //    simd3       28       29        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd       46       57        0      N/A
    //  no simd      109      125        0        0
    fn mul(self, other: Circle) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<CircleRotor> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       20       27        0        0
    //    simd2        7       10        0      N/A
    //    simd3       26       27        0      N/A
    //    simd4        4        4        0      N/A
    // Totals...
    // yes simd       57       68        0      N/A
    //  no simd      128      144        0        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Dipole> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       15        0        0
    //    simd2        3        6        0      N/A
    //    simd3       28       29        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd       43       52        0      N/A
    //  no simd      111      122        0        0
    fn mul(self, other: Dipole) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<DipoleInversion> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       29       36        0        0
    //    simd2       18       21        0      N/A
    //    simd3       34       35        0      N/A
    // Totals...
    // yes simd       81       92        0      N/A
    //  no simd      167      183        0        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<DualNum> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        1        5        0      N/A
    //    simd4        3        3        0      N/A
    // Totals...
    // yes simd        4       10        0      N/A
    //  no simd       15       29        0        0
    fn mul(self, other: DualNum) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<FlatPoint> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       21        0        0
    //    simd3        7       10        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd       21       32        0      N/A
    //  no simd       38       55        0        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Flector> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       24       31        0        0
    //    simd2       11       14        0      N/A
    //    simd3       13       14        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd       50       61        0      N/A
    //  no simd       93      109        0        0
    fn mul(self, other: Flector) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Line> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3       18       22        0      N/A
    // Totals...
    // yes simd       23       28        0      N/A
    //  no simd       59       72        0        0
    fn mul(self, other: Line) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Motor> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       19        0        0
    //    simd2        9       12        0      N/A
    //    simd3       14       15        0      N/A
    //    simd4        3        3        0      N/A
    // Totals...
    // yes simd       37       49        0      N/A
    //  no simd       83      100        0        0
    fn mul(self, other: Motor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<MultiVector> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       97      115        0        0
    //    simd2       18       24        0      N/A
    //    simd3       83       83        0      N/A
    //    simd4       11       11        0      N/A
    // Totals...
    // yes simd      209      233        0      N/A
    //  no simd      426      456        0        0
    fn mul(self, other: MultiVector) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Plane> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        7        0        0
    //    simd2        2        5        0      N/A
    //    simd3        8        9        0      N/A
    //    simd4        4        1        0      N/A
    // Totals...
    // yes simd       16       22        0      N/A
    //  no simd       46       48        0        0
    fn mul(self, other: Plane) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<RoundPoint> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd2        2        5        0      N/A
    //    simd3        9       10        0      N/A
    //    simd4        6        4        0      N/A
    // Totals...
    // yes simd       20       27        0      N/A
    //  no simd       58       64        0        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Scalar> for DipoleInversion {
    type Output = DipoleInversion;
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
impl std::ops::MulAssign<Scalar> for DipoleInversion {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        8        0        0
    //    simd2        2        5        0      N/A
    //    simd3       10       11        0      N/A
    //    simd4        5        3        0      N/A
    // Totals...
    // yes simd       19       27        0      N/A
    //  no simd       56       63        0        0
    fn mul(self, other: Sphere) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<VersorEven> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       55       63        0        0
    //    simd2       28       32        0      N/A
    //    simd3       23       23        0      N/A
    //    simd4        5        5        0      N/A
    // Totals...
    // yes simd      111      123        0      N/A
    //  no simd      200      216        0        0
    fn mul(self, other: VersorEven) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<VersorOdd> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       38       46        0        0
    //    simd2       25       29        0      N/A
    //    simd3       30       30        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd       94      106        0      N/A
    //  no simd      182      198        0        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Neg for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       15        0        0
    fn neg(self) -> Self::Output {
        DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group3() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Not for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       15        0        0
    fn not(self) -> Self::Output {
        self.right_dual()
    }
}
impl std::ops::Sub<AntiCircleRotor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        1        0        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        3        1        0      N/A
    //  no simd       11        1        0        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0() - other.group0()).with_w(other[scalar] * -1.0),
            // e23, e31, e12, e45
            self.group1() - other.group1(),
            // e15, e25, e35, e1234
            self.group2() + -other.group2().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        )
    }
}
impl std::ops::Sub<AntiDipoleInversion> for DipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (other.group3().xyz() * Simd32x3::from(-1.0)).with_w(other[e4] * -1.0),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            self.group0(),
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
impl std::ops::Sub<AntiDualNum> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        1        1        0        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(other[scalar] * -1.0),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3().xyz().with_w(self[e3215] - other[e3215]),
        )
    }
}
impl std::ops::Sub<AntiFlatPoint> for DipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            self.group0(),
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
impl std::ops::Sub<AntiFlector> for DipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            self.group0(),
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
impl std::ops::Sub<AntiLine> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        2        0        0      N/A
    // no simd        8        0        0        0
    fn sub(self, other: AntiLine) -> Self::Output {
        DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() + -other.group0().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2() + -other.group1().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        )
    }
}
impl std::ops::SubAssign<AntiLine> for DipoleInversion {
    fn sub_assign(&mut self, other: AntiLine) {
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() + -other.group0().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2() + -other.group1().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<AntiMotor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        1        0        0
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        3        1        0      N/A
    //  no simd        9        1        0        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(other[scalar] * -1.0),
            // e23, e31, e12, e45
            self.group1() + -other.group0().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2() + -other.group1().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3().xyz().with_w(self[e3215] - other[e3215]),
        )
    }
}
impl std::ops::Sub<AntiPlane> for DipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            self.group0(),
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
impl std::ops::Sub<AntiScalar> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            self.group0(),
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
impl std::ops::Sub<Circle> for DipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            self.group0(),
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
impl std::ops::Sub<CircleRotor> for DipoleInversion {
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
            Simd32x2::from([0.0, other[e12345] * -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            self.group0(),
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
impl std::ops::Sub<Dipole> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        1        0        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        3        0        0      N/A
    //  no simd       11        0        0        0
    fn sub(self, other: Dipole) -> Self::Output {
        DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() - other.group0(),
            // e23, e31, e12, e45
            self.group1() - other.group1(),
            // e15, e25, e35, e1234
            self.group2() + -other.group2().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        )
    }
}
impl std::ops::SubAssign<Dipole> for DipoleInversion {
    fn sub_assign(&mut self, other: Dipole) {
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() - other.group0(),
            // e23, e31, e12, e45
            self.group1() - other.group1(),
            // e15, e25, e35, e1234
            self.group2() + -other.group2().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<DipoleInversion> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        1        0        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        4        0        0      N/A
    //  no simd       15        0        0        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        DipoleInversion::from_groups(
            // e41, e42, e43
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
impl std::ops::SubAssign<DipoleInversion> for DipoleInversion {
    fn sub_assign(&mut self, other: DipoleInversion) {
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
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
impl std::ops::Sub<DualNum> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn sub(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            self.group0(),
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
impl std::ops::Sub<FlatPoint> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        5        0        0        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e45] - other[e45]),
            // e15, e25, e35, e1234
            self.group2() + -other.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        )
    }
}
impl std::ops::SubAssign<FlatPoint> for DipoleInversion {
    fn sub_assign(&mut self, other: FlatPoint) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e45] - other[e45]),
            // e15, e25, e35, e1234
            self.group2() + -other.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<Flector> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        3        0        0      N/A
    //  no simd        9        0        0        0
    fn sub(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e45] - other[e45]),
            // e15, e25, e35, e1234
            self.group2() + -other.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3() - other.group1(),
        )
    }
}
impl std::ops::SubAssign<Flector> for DipoleInversion {
    fn sub_assign(&mut self, other: Flector) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e45] - other[e45]),
            // e15, e25, e35, e1234
            self.group2() + -other.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3() - other.group1(),
        );
    }
}
impl std::ops::Sub<Line> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn sub(self, other: Line) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            self.group0(),
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
impl std::ops::Sub<Motor> for DipoleInversion {
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
            Simd32x2::from([0.0, other[e12345] * -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            self.group0(),
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
impl std::ops::Sub<MultiVector> for DipoleInversion {
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
            other.group1() * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            self.group2().xyz().with_w(self[e45]) - other.group3(),
            // e41, e42, e43
            self.group0() - other.group4(),
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
impl std::ops::Sub<Plane> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        1        0        0      N/A
    // no simd        4        0        0        0
    fn sub(self, other: Plane) -> Self::Output {
        DipoleInversion::from_groups(
            // e41, e42, e43
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
impl std::ops::SubAssign<Plane> for DipoleInversion {
    fn sub_assign(&mut self, other: Plane) {
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
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
impl std::ops::Sub<RoundPoint> for DipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group0() * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            self.group0(),
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
impl std::ops::Sub<Scalar> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(other[scalar] * -1.0),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        )
    }
}
impl std::ops::Sub<Sphere> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        5        0        0        0
    fn sub(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
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
impl std::ops::SubAssign<Sphere> for DipoleInversion {
    fn sub_assign(&mut self, other: Sphere) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
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
impl std::ops::Sub<VersorEven> for DipoleInversion {
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
            Simd32x2::from([0.0, other[e12345] * -1.0]),
            // e1, e2, e3, e4
            other.group3() * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            self.group0(),
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
impl std::ops::Sub<VersorOdd> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        1        0        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        4        4        0      N/A
    //  no simd       15        4        0        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x3::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0]) + self.group0()).with_w(other[scalar] * -1.0),
            // e23, e31, e12, e45
            self.group1() - other.group1(),
            // e15, e25, e35, e1234
            self.group2() - other.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() - other.group3(),
        )
    }
}

impl TryFrom<AntiCircleRotor> for DipoleInversion {
    type Error = String;
    fn try_from(anti_circle_rotor: AntiCircleRotor) -> Result<Self, Self::Error> {
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
        Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            anti_circle_rotor.group0(),
            // e23, e31, e12, e45
            anti_circle_rotor.group1(),
            // e15, e25, e35, e1234
            anti_circle_rotor.group2().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ))
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
        Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(anti_dual_num[e3215]),
        ))
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
        Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            anti_motor.group0().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            anti_motor.group1().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(anti_motor[e3215]),
        ))
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
        Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            multi_vector.group4(),
            // e23, e31, e12, e45
            multi_vector.group5().with_w(multi_vector[e45]),
            // e15, e25, e35, e1234
            multi_vector.group3().xyz().with_w(multi_vector[e1234]),
            // e4235, e4315, e4125, e3215
            multi_vector.group9(),
        ))
    }
}

impl TryFrom<VersorOdd> for DipoleInversion {
    type Error = String;
    fn try_from(versor_odd: VersorOdd) -> Result<Self, Self::Error> {
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
        Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            versor_odd.group0().xyz(),
            // e23, e31, e12, e45
            versor_odd.group1(),
            // e15, e25, e35, e1234
            versor_odd.group2(),
            // e4235, e4315, e4125, e3215
            versor_odd.group3(),
        ))
    }
}
