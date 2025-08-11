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
// Total Implementations: 105
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         1       2       0     N/A
//  Average:         6       8       0     N/A
//  Maximum:       106     130       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         4       6       0       0
//  Average:        13      17       0       0
//  Maximum:       214     245       0       0
impl std::ops::Add<AntiCircleRotor> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd3        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        4        0        0        0
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0().with_w(other[scalar]),
            // e23, e31, e12, e45
            other.group1().xyz().with_w(other[e45] + self[e45]),
            // e15, e25, e35, e1234
            (other.group2().xyz() + self.group0().xyz()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group1(),
        )
    }
}
impl std::ops::Add<AntiDipoleInversion> for Flector {
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
            self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2().xyz(),
            // e4235, e4315, e4125, e3215
            self.group1(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<AntiDualNum> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        1        0        0        0
    fn add(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(other[scalar]),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(self[e45]),
            // e15, e25, e35, e1234
            self.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group1().xyz().with_w(other[e3215] + self[e3215]),
        )
    }
}
impl std::ops::Add<AntiFlatPoint> for Flector {
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
            self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().xyz(),
            // e4235, e4315, e4125, e3215
            self.group1(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<AntiFlector> for Flector {
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
            self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().xyz(),
            // e4235, e4315, e4125, e3215
            self.group1(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<AntiLine> for Flector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        1        0        0      N/A
    // no simd        3        0        0        0
    fn add(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            other.group0().with_w(self[e45]),
            // e15, e25, e35, e1234
            (other.group1() + self.group0().xyz()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group1(),
        )
    }
}
impl std::ops::Add<AntiMotor> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd3        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        4        0        0        0
    fn add(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(other[scalar]),
            // e23, e31, e12, e45
            other.group0().xyz().with_w(self[e45]),
            // e15, e25, e35, e1234
            (other.group1().xyz() + self.group0().xyz()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group1().xyz().with_w(other[e3215] + self[e3215]),
        )
    }
}
impl std::ops::Add<AntiPlane> for Flector {
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
            self.group0(),
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
            self.group1(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<AntiScalar> for Flector {
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
            self.group0(),
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
            self.group1(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<Circle> for Flector {
    type Output = MultiVector;
    fn add(self, other: Circle) -> Self::Output {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2(),
            // e4235, e4315, e4125, e3215
            self.group1(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<CircleRotor> for Flector {
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
            self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2().xyz(),
            // e4235, e4315, e4125, e3215
            self.group1(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<Dipole> for Flector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd3        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        4        0        0        0
    fn add(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1().xyz().with_w(other[e45] + self[e45]),
            // e15, e25, e35, e1234
            (other.group2() + self.group0().xyz()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group1(),
        )
    }
}
impl std::ops::Add<DipoleInversion> for Flector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        3        0        0      N/A
    //  no simd        9        0        0        0
    fn add(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1().xyz().with_w(other[e45] + self[e45]),
            // e15, e25, e35, e1234
            other.group2() + self.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            other.group3() + self.group1(),
        )
    }
}
impl std::ops::Add<DualNum> for Flector {
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
            self.group0(),
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
            self.group1(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<FlatPoint> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        1        0        0      N/A
    // no simd        4        0        0        0
    fn add(self, other: FlatPoint) -> Self::Output {
        Flector::from_groups(/* e15, e25, e35, e45 */ other.group0() + self.group0(), /* e4235, e4315, e4125, e3215 */ self.group1())
    }
}
impl std::ops::AddAssign<FlatPoint> for Flector {
    fn add_assign(&mut self, other: FlatPoint) {
        *self = Flector::from_groups(/* e15, e25, e35, e45 */ other.group0() + self.group0(), /* e4235, e4315, e4125, e3215 */ self.group1());
    }
}
impl std::ops::Add<Flector> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        2        0        0      N/A
    // no simd        8        0        0        0
    fn add(self, other: Flector) -> Self::Output {
        Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() + self.group0(),
            // e4235, e4315, e4125, e3215
            other.group1() + self.group1(),
        )
    }
}
impl std::ops::AddAssign<Flector> for Flector {
    fn add_assign(&mut self, other: Flector) {
        *self = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() + self.group0(),
            // e4235, e4315, e4125, e3215
            other.group1() + self.group1(),
        );
    }
}
impl std::ops::Add<Line> for Flector {
    type Output = MultiVector;
    fn add(self, other: Line) -> Self::Output {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e4235, e4315, e4125, e3215
            self.group1(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<Motor> for Flector {
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
            self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1().xyz(),
            // e4235, e4315, e4125, e3215
            self.group1(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        2        0        0      N/A
    // no simd        8        0        0        0
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
            self.group0() + other.group3(),
            // e41, e42, e43
            other.group4(),
            // e23, e31, e12
            other.group5(),
            // e415, e425, e435, e321
            other.group6(),
            // e423, e431, e412
            other.group7(),
            // e235, e315, e125
            other.group8(),
            // e4235, e4315, e4125, e3215
            self.group1() + other.group9(),
            // e1234
            other[e1234],
        )
    }
}
impl std::ops::Add<Plane> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        1        0        0      N/A
    // no simd        4        0        0        0
    fn add(self, other: Plane) -> Self::Output {
        Flector::from_groups(/* e15, e25, e35, e45 */ self.group0(), /* e4235, e4315, e4125, e3215 */ self.group1() + other.group0())
    }
}
impl std::ops::AddAssign<Plane> for Flector {
    fn add_assign(&mut self, other: Plane) {
        *self = Flector::from_groups(/* e15, e25, e35, e45 */ self.group0(), /* e4235, e4315, e4125, e3215 */ self.group1() + other.group0());
    }
}
impl std::ops::Add<RoundPoint> for Flector {
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
            self.group0(),
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
            self.group1(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<Scalar> for Flector {
    type Output = VersorOdd;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(other[scalar]),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(self[e45]),
            // e15, e25, e35, e1234
            self.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group1(),
        )
    }
}
impl std::ops::Add<Sphere> for Flector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        1        0        0      N/A
    // no simd        4        0        0        0
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(self[e45]),
            // e15, e25, e35, e1234
            self.group0().xyz().with_w(other[e1234]),
            // e4235, e4315, e4125, e3215
            self.group1() + other.group0(),
        )
    }
}
impl std::ops::Add<VersorEven> for Flector {
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
            self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            other.group2().xyz(),
            // e4235, e4315, e4125, e3215
            self.group1(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<VersorOdd> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd3        1        0        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        3        0        0      N/A
    //  no simd        8        0        0        0
    fn add(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0(),
            // e23, e31, e12, e45
            other.group1().xyz().with_w(self[e45] + other[e45]),
            // e15, e25, e35, e1234
            (self.group0().xyz() + other.group2().xyz()).with_w(other[e1234]),
            // e4235, e4315, e4125, e3215
            self.group1() + other.group3(),
        )
    }
}
impl std::ops::BitXor<AntiCircleRotor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        3        4        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        9       17        0        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<AntiCircleRotor> for Flector {
    fn bitxor_assign(&mut self, other: AntiCircleRotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for Flector {
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
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<AntiDualNum> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn bitxor(self, other: AntiDualNum) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<AntiDualNum> for Flector {
    fn bitxor_assign(&mut self, other: AntiDualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatPoint> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn bitxor(self, other: AntiFlatPoint) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<AntiFlector> for Flector {
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
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<AntiLine> for Flector {
    type Output = Plane;
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
impl std::ops::BitXor<AntiMotor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd        1        3        0      N/A
    //  no simd        4       11        0        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<AntiMotor> for Flector {
    fn bitxor_assign(&mut self, other: AntiMotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd2        1        2        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        5        9        0      N/A
    //  no simd        9       15        0        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Circle> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn bitxor(self, other: Circle) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<CircleRotor> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn bitxor(self, other: CircleRotor) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Dipole> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        3        0      N/A
    // no simd        6        9        0        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<DipoleInversion> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        3        0      N/A
    // no simd        6        9        0        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       14        0        0
    //    simd3        3        5        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd       14       21        0      N/A
    //  no simd       23       37        0        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<RoundPoint> for Flector {
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
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Scalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<Scalar> for Flector {
    fn bitxor_assign(&mut self, other: Scalar) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for Flector {
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
    fn bitxor(self, other: VersorEven) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<VersorOdd> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd       14       18        0        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<VersorOdd> for Flector {
    fn bitxor_assign(&mut self, other: VersorOdd) {
        *self = self.wedge(other);
    }
}

impl From<FlatPoint> for Flector {
    fn from(from_flat_point: FlatPoint) -> Self {
        Flector::from_groups(/* e15, e25, e35, e45 */ from_flat_point.group0(), /* e4235, e4315, e4125, e3215 */ Simd32x4::from(0.0))
    }
}

impl From<Plane> for Flector {
    fn from(from_plane: Plane) -> Self {
        Flector::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(0.0), /* e4235, e4315, e4125, e3215 */ from_plane.group0())
    }
}
impl std::ops::Mul<AntiCircleRotor> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       18        0        0
    //    simd2        7        9        0      N/A
    //    simd3       11       13        0      N/A
    // Totals...
    // yes simd       30       40        0      N/A
    //  no simd       59       75        0        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiDipoleInversion> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       19       26        0        0
    //    simd2       10       13        0      N/A
    //    simd3       14       15        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd       45       56        0      N/A
    //  no simd       89      105        0        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiDualNum> for Flector {
    type Output = Flector;
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
impl std::ops::MulAssign<AntiDualNum> for Flector {
    fn mul_assign(&mut self, other: AntiDualNum) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd2        1        2        0      N/A
    //    simd3        1        1        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd       14       18        0        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiFlector> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        8        0        0
    //    simd2        5        7        0      N/A
    //    simd3        2        2        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd       16       20        0      N/A
    //  no simd       37       40        0        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiLine> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        7        9        0      N/A
    // no simd       21       27        0        0
    fn mul(self, other: AntiLine) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<AntiLine> for Flector {
    fn mul_assign(&mut self, other: AntiLine) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd2        4        6        0      N/A
    //    simd3        3        3        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd       17       21        0      N/A
    //  no simd       39       42        0        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<AntiMotor> for Flector {
    fn mul_assign(&mut self, other: AntiMotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd2        2        4        0      N/A
    //    simd3        1        1        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd        9       11        0      N/A
    //  no simd       25       23        0        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiScalar> for Flector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Circle> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       13        0        0
    //    simd2        4        6        0      N/A
    //    simd3       12       14        0      N/A
    // Totals...
    // yes simd       22       33        0      N/A
    //  no simd       50       67        0        0
    fn mul(self, other: Circle) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<CircleRotor> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       16        0        0
    //    simd2        6        8        0      N/A
    //    simd3       11       13        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd       27       38        0      N/A
    //  no simd       58       75        0        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Dipole> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       13        0        0
    //    simd2        4        6        0      N/A
    //    simd3       12       14        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd       26       33        0      N/A
    //  no simd       57       67        0        0
    fn mul(self, other: Dipole) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<DipoleInversion> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       25       32        0        0
    //    simd2       11       14        0      N/A
    //    simd3       14       15        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd       51       62        0      N/A
    //  no simd       93      109        0        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<DualNum> for Flector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        5        0        0
    //    simd3        1        4        0      N/A
    // Totals...
    // yes simd        2        9        0      N/A
    //  no simd        4       17        0        0
    fn mul(self, other: DualNum) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<FlatPoint> for Flector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd2        2        3        0      N/A
    //    simd3        1        1        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd        6        8        0      N/A
    //  no simd       13       16        0        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Flector> for Flector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       11        0        0
    //    simd2        5        7        0      N/A
    //    simd4        6        5        0      N/A
    // Totals...
    // yes simd       19       23        0      N/A
    //  no simd       42       45        0        0
    fn mul(self, other: Flector) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Line> for Flector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        7        9        0      N/A
    // Totals...
    // yes simd       11       15        0      N/A
    //  no simd       25       33        0        0
    fn mul(self, other: Line) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Motor> for Flector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       10        0        0
    //    simd2        3        5        0      N/A
    //    simd3        2        2        0      N/A
    //    simd4        5        5        0      N/A
    // Totals...
    // yes simd       16       22        0      N/A
    //  no simd       38       46        0        0
    fn mul(self, other: Motor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       52       70        0        0
    //    simd2        8       13        0      N/A
    //    simd3       38       39        0      N/A
    //    simd4        8        8        0      N/A
    // Totals...
    // yes simd      106      130        0      N/A
    //  no simd      214      245        0        0
    fn mul(self, other: MultiVector) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Plane> for Flector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd2        1        3        0      N/A
    //    simd3        2        2        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       25       24        0        0
    fn mul(self, other: Plane) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<RoundPoint> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       15        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4        6        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd       12       24        0      N/A
    //  no simd       26       43        0        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Scalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn mul(self, other: Scalar) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Scalar> for Flector {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       24        0        0
    //    simd3        4        6        0      N/A
    // Totals...
    // yes simd       16       30        0      N/A
    //  no simd       24       42        0        0
    fn mul(self, other: Sphere) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<VersorEven> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       15       22        0        0
    //    simd2       12       16        0      N/A
    //    simd3        5        5        0      N/A
    //    simd4       12       11        0      N/A
    // Totals...
    // yes simd       44       54        0      N/A
    //  no simd      102      113        0        0
    fn mul(self, other: VersorEven) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<VersorOdd> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       19       27        0        0
    //    simd2        9       13        0      N/A
    //    simd3       12       12        0      N/A
    //    simd4        7        7        0      N/A
    // Totals...
    // yes simd       47       59        0      N/A
    //  no simd      101      117        0        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Neg for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn neg(self) -> Self::Output {
        Flector::from_groups(
            // e15, e25, e35, e45
            self.group0() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Not for Flector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn not(self) -> Self::Output {
        self.right_dual()
    }
}
impl std::ops::Sub<AntiCircleRotor> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        1        0        0
    //    simd3        1        2        0      N/A
    // Totals...
    // yes simd        2        3        0      N/A
    //  no simd        4        7        0        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (other.group0() * Simd32x3::from(-1.0)).with_w(other[scalar] * -1.0),
            // e23, e31, e12, e45
            (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(self[e45] - other[e45]),
            // e15, e25, e35, e1234
            (self.group0().xyz() - other.group2().xyz()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group1(),
        )
    }
}
impl std::ops::Sub<AntiDipoleInversion> for Flector {
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
            self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group1(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<AntiDualNum> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        1        1        0        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(self[e45]),
            // e15, e25, e35, e1234
            self.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group1().xyz().with_w(self[e3215] - other[e3215]),
        )
    }
}
impl std::ops::Sub<AntiFlatPoint> for Flector {
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
            self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group1(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<AntiFlector> for Flector {
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
            self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group1(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<AntiLine> for Flector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        1        1        0      N/A
    // no simd        3        3        0        0
    fn sub(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            (other.group0() * Simd32x3::from(-1.0)).with_w(self[e45]),
            // e15, e25, e35, e1234
            (self.group0().xyz() - other.group1()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group1(),
        )
    }
}
impl std::ops::Sub<AntiMotor> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        1        0        0
    //    simd3        1        1        0      N/A
    // Totals...
    // yes simd        2        2        0      N/A
    //  no simd        4        4        0        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
            // e23, e31, e12, e45
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e45]),
            // e15, e25, e35, e1234
            (self.group0().xyz() - other.group1().xyz()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group1().xyz().with_w(self[e3215] - other[e3215]),
        )
    }
}
impl std::ops::Sub<AntiPlane> for Flector {
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
            self.group0(),
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
            self.group1(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<AntiScalar> for Flector {
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
            self.group0(),
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
            self.group1(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<Circle> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       10        0        0
    fn sub(self, other: Circle) -> Self::Output {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group1(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<CircleRotor> for Flector {
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
            self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group1(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<Dipole> for Flector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd3        1        2        0      N/A
    // Totals...
    // yes simd        2        2        0      N/A
    //  no simd        4        6        0        0
    fn sub(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(self[e45] - other[e45]),
            // e15, e25, e35, e1234
            (self.group0().xyz() - other.group2()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group1(),
        )
    }
}
impl std::ops::Sub<DipoleInversion> for Flector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        1        0        0
    //    simd2        1        0        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        4        3        0      N/A
    //  no simd        8        7        0        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(self[e45] - other[e45]),
            // e15, e25, e35, e1234
            (self.group0().xy() - other.group2().xy()).with_zw(self[e35] - other[e35], other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            self.group1() - other.group3(),
        )
    }
}
impl std::ops::Sub<DualNum> for Flector {
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
            self.group0(),
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
            self.group1(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<FlatPoint> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        1        0        0      N/A
    // no simd        4        0        0        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        Flector::from_groups(/* e15, e25, e35, e45 */ self.group0() - other.group0(), /* e4235, e4315, e4125, e3215 */ self.group1())
    }
}
impl std::ops::SubAssign<FlatPoint> for Flector {
    fn sub_assign(&mut self, other: FlatPoint) {
        *self = Flector::from_groups(/* e15, e25, e35, e45 */ self.group0() - other.group0(), /* e4235, e4315, e4125, e3215 */ self.group1());
    }
}
impl std::ops::Sub<Flector> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        2        0        0      N/A
    // no simd        8        0        0        0
    fn sub(self, other: Flector) -> Self::Output {
        Flector::from_groups(
            // e15, e25, e35, e45
            self.group0() - other.group0(),
            // e4235, e4315, e4125, e3215
            self.group1() - other.group1(),
        )
    }
}
impl std::ops::SubAssign<Flector> for Flector {
    fn sub_assign(&mut self, other: Flector) {
        *self = Flector::from_groups(
            // e15, e25, e35, e45
            self.group0() - other.group0(),
            // e4235, e4315, e4125, e3215
            self.group1() - other.group1(),
        );
    }
}
impl std::ops::Sub<Line> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn sub(self, other: Line) -> Self::Output {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group1(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<Motor> for Flector {
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
            self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group1(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        4        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        2        9        0      N/A
    //  no simd        8       24        0        0
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
            self.group0() - other.group3(),
            // e41, e42, e43
            other.group4() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group5() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group6() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group7() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group8() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group1() - other.group9(),
            // e1234
            other[e1234] * -1.0,
        )
    }
}
impl std::ops::Sub<Plane> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        1        0        0      N/A
    // no simd        4        0        0        0
    fn sub(self, other: Plane) -> Self::Output {
        Flector::from_groups(/* e15, e25, e35, e45 */ self.group0(), /* e4235, e4315, e4125, e3215 */ self.group1() - other.group0())
    }
}
impl std::ops::SubAssign<Plane> for Flector {
    fn sub_assign(&mut self, other: Plane) {
        *self = Flector::from_groups(/* e15, e25, e35, e45 */ self.group0(), /* e4235, e4315, e4125, e3215 */ self.group1() - other.group0());
    }
}
impl std::ops::Sub<RoundPoint> for Flector {
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
            self.group0(),
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
            self.group1(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<Scalar> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(self[e45]),
            // e15, e25, e35, e1234
            self.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group1(),
        )
    }
}
impl std::ops::Sub<Sphere> for Flector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        1        1        0      N/A
    //  no simd        4        1        0        0
    fn sub(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(self[e45]),
            // e15, e25, e35, e1234
            self.group0().xyz().with_w(other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            self.group1() - other.group0(),
        )
    }
}
impl std::ops::Sub<VersorEven> for Flector {
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
            self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group1(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<VersorOdd> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        1        0        0
    //    simd2        1        0        0      N/A
    //    simd3        0        1        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd        4        3        0      N/A
    //  no simd        8        8        0        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12, e45
            (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(self[e45] - other[e45]),
            // e15, e25, e35, e1234
            (self.group0().xy() - other.group2().xy()).with_zw(self[e35] - other[e35], other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            self.group1() - other.group3(),
        )
    }
}

impl TryFrom<AntiCircleRotor> for Flector {
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
        let el = anti_circle_rotor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotor do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(Flector::from_groups(
            // e15, e25, e35, e45
            anti_circle_rotor.group2().xyz().with_w(anti_circle_rotor[e45]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<AntiDualNum> for Flector {
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
            let mut error = "Elements from AntiDualNum do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(anti_dual_num[e3215]),
        ))
    }
}

impl TryFrom<AntiLine> for Flector {
    type Error = String;
    fn try_from(anti_line: AntiLine) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_line[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_line[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_line[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiLine do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(Flector::from_groups(
            // e15, e25, e35, e45
            anti_line.group1().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<AntiMotor> for Flector {
    type Error = String;
    fn try_from(anti_motor: AntiMotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_motor[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_motor[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_motor[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_motor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMotor do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(Flector::from_groups(
            // e15, e25, e35, e45
            anti_motor.group1().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(anti_motor[e3215]),
        ))
    }
}

impl TryFrom<Dipole> for Flector {
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
        let el = dipole[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Dipole do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(Flector::from_groups(
            // e15, e25, e35, e45
            dipole.group2().with_w(dipole[e45]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<DipoleInversion> for Flector {
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
        let el = dipole_inversion[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
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
        if fail {
            let mut error = "Elements from DipoleInversion do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(Flector::from_groups(
            // e15, e25, e35, e45
            dipole_inversion.group2().xyz().with_w(dipole_inversion[e45]),
            // e4235, e4315, e4125, e3215
            dipole_inversion.group3(),
        ))
    }
}

impl TryFrom<MultiVector> for Flector {
    type Error = String;
    fn try_from(multi_vector: MultiVector) -> Result<Self, Self::Error> {
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
        let el = multi_vector[31];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MultiVector do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(Flector::from_groups(
            // e15, e25, e35, e45
            multi_vector.group3(),
            // e4235, e4315, e4125, e3215
            multi_vector.group9(),
        ))
    }
}

impl TryFrom<Sphere> for Flector {
    type Error = String;
    fn try_from(sphere: Sphere) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = sphere[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Sphere do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            sphere.group0(),
        ))
    }
}

impl TryFrom<VersorOdd> for Flector {
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
        let el = versor_odd[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
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
        if fail {
            let mut error = "Elements from VersorOdd do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(Flector::from_groups(
            // e15, e25, e35, e45
            versor_odd.group2().xyz().with_w(versor_odd[e45]),
            // e4235, e4315, e4125, e3215
            versor_odd.group3(),
        ))
    }
}
