use crate::traits::GeometricProduct;
use crate::traits::RightDual;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 331
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         1       2       0
//  Average:         8      13       0
//  Maximum:       144     174       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       4       0
//  Average:        21      25       0
//  Maximum:       352     384       0
impl std::ops::Add<AntiCircleOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiCircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotor> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(other[e45]),
            // e15, e25, e35
            other.group2().xyz(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(0.0),
            // e15, e25, e35
            other.group2().xyz(),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOriginAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1().xyz(),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            other.group1().xyz(),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().xyz().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiDipoleInversion> for VersorEvenOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       15        0        0
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() + self.group0().xyz(),
            // e415, e425, e435, e321
            other.group1() + Simd32x3::from(0.0).with_w(self[e321]),
            // e235, e315, e125, e4
            other.group2() + self.group1().xyz().with_w(self[e4]),
            // e1, e2, e3, e5
            other.group3() + self.group2().xyz().with_w(self[e5]),
        );
    }
}
impl std::ops::Add<AntiDipoleInversionAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       11        0        0
    fn add(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            other.group0() + Simd32x3::from(0.0).with_w(self[e321]),
            // e235, e315, e125, e4
            (other.group1() + self.group1().xyz()).with_w(self[e4]),
            // e1, e2, e3, e5
            other.group2() + self.group2().xyz().with_w(self[e5]),
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            other.group0() + self.group0(),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2() + other.group1().yzwx(),
        );
    }
}
impl std::ops::AddAssign<AntiDipoleInversionOnOrigin> for VersorEvenOrthogonalOrigin {
    fn add_assign(&mut self, other: AntiDipoleInversionOnOrigin) {
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            other.group0() + self.group0(),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2() + other.group1().yzwx(),
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOrthogonalOrigin> for VersorEvenOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        8        0        0
    fn add(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0().xyz() + self.group0().xyz(),
            // e415, e425, e435, e321
            other.group1().with_w(self[e321]),
            // e235, e315, e125, e4
            other.group2() + self.group1().xyz().with_w(self[e4]),
            // e1, e2, e3, e5
            self.group2().xyz().with_w(other[e5] + self[e5]),
        );
    }
}
impl std::ops::Add<AntiDipoleOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            other.group0() + self.group0(),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<AntiDipoleOnOrigin> for VersorEvenOrthogonalOrigin {
    fn add_assign(&mut self, other: AntiDipoleOnOrigin) {
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            other.group0() + self.group0(),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiDualNum> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiFlatOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321]),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<AntiFlatOrigin> for VersorEvenOrthogonalOrigin {
    fn add_assign(&mut self, other: AntiFlatOrigin) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321]),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiFlatPoint> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0().xyz().with_w(other[e321] + self[e321]),
            // e235, e315, e125, e5
            (other.group0().xyz() + self.group1().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<AntiFlatPoint> for VersorEvenOrthogonalOrigin {
    fn add_assign(&mut self, other: AntiFlatPoint) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0().xyz().with_w(other[e321] + self[e321]),
            // e235, e315, e125, e5
            (other.group0().xyz() + self.group1().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiFlector> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        8        0        0
    fn add(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0().xyz().with_w(other[e321] + self[e321]),
            // e235, e315, e125, e5
            self.group1() + other.group0().xyz().with_w(other[e5]),
            // e1, e2, e3, e4
            (other.group1().xyz() + self.group2().xyz()).with_w(self[e4]),
        );
    }
}
impl std::ops::AddAssign<AntiFlector> for VersorEvenOrthogonalOrigin {
    fn add_assign(&mut self, other: AntiFlector) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0().xyz().with_w(other[e321] + self[e321]),
            // e235, e315, e125, e5
            self.group1() + other.group0().xyz().with_w(other[e5]),
            // e1, e2, e3, e4
            (other.group1().xyz() + self.group2().xyz()).with_w(self[e4]),
        );
    }
}
impl std::ops::Add<AntiFlectorOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0().xyz().with_w(other[e321] + self[e321]),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            (self.group2().xyz() + other.group0().yzw()).with_w(self[e4]),
        );
    }
}
impl std::ops::AddAssign<AntiFlectorOnOrigin> for VersorEvenOrthogonalOrigin {
    fn add_assign(&mut self, other: AntiFlectorOnOrigin) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0().xyz().with_w(other[e321] + self[e321]),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            (self.group2().xyz() + other.group0().yzw()).with_w(self[e4]),
        );
    }
}
impl std::ops::Add<AntiLine> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiLineOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiLineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiMotor> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1().xyz(),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<AntiMotorOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiMysteryCircleRotor> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiMysteryDipoleInversion> for VersorEvenOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            other.group0() + Simd32x3::from(0.0).with_w(self[e321]),
            // e235, e315, e125, e4
            Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            (other.group1() + self.group2().xyz()).with_w(self[e5]),
        );
    }
}
impl std::ops::Add<AntiPlane> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1().xyz().with_w(other[e5] + self[e5]),
            // e1, e2, e3, e4
            (other.group0().xyz() + self.group2().xyz()).with_w(self[e4]),
        );
    }
}
impl std::ops::AddAssign<AntiPlane> for VersorEvenOrthogonalOrigin {
    fn add_assign(&mut self, other: AntiPlane) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1().xyz().with_w(other[e5] + self[e5]),
            // e1, e2, e3, e4
            (other.group0().xyz() + self.group2().xyz()).with_w(self[e4]),
        );
    }
}
impl std::ops::Add<AntiPlaneOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: AntiPlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            (other.group0() + self.group2().xyz()).with_w(self[e4]),
        );
    }
}
impl std::ops::AddAssign<AntiPlaneOnOrigin> for VersorEvenOrthogonalOrigin {
    fn add_assign(&mut self, other: AntiPlaneOnOrigin) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            (other.group0() + self.group2().xyz()).with_w(self[e4]),
        );
    }
}
impl std::ops::Add<AntiScalar> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiSphereOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: AntiSphereOnOrigin) -> Self::Output {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            other.group0() + self.group2(),
        );
    }
}
impl std::ops::AddAssign<AntiSphereOnOrigin> for VersorEvenOrthogonalOrigin {
    fn add_assign(&mut self, other: AntiSphereOnOrigin) {
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            other.group0() + self.group2(),
        );
    }
}
impl std::ops::Add<AntiVersorEvenOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().xyz().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Circle> for VersorEvenOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn add(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() + self.group0().xyz(),
            // e415, e425, e435, e321
            other.group1() + Simd32x3::from(0.0).with_w(self[e321]),
            // e235, e315, e125, e4
            (other.group2() + self.group1().xyz()).with_w(self[e4]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl std::ops::Add<CircleAligningOrigin> for VersorEvenOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: CircleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() + self.group0().xyz(),
            // e415, e425, e435, e321
            other.group1().with_w(self[e321]),
            // e235, e315, e125, e4
            (other.group2() + self.group1().xyz()).with_w(self[e4]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl std::ops::Add<CircleAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: CircleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            other.group0() + Simd32x3::from(0.0).with_w(self[e321]),
            // e235, e315, e125, e4
            (other.group1() + self.group1().xyz()).with_w(self[e4]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl std::ops::Add<CircleAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: CircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            (other.group0() + self.group0().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            (other.group1() + self.group1().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<CircleAtOrigin> for VersorEvenOrthogonalOrigin {
    fn add_assign(&mut self, other: CircleAtOrigin) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            (other.group0() + self.group0().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            (other.group1() + self.group1().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Add<CircleOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: CircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() + self.group0().xyz(),
            // e415, e425, e435, e321
            other.group1().with_w(self[e321]),
            // e235, e315, e125, e4
            Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl std::ops::Add<CircleOrthogonalOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: CircleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            other.group0() + self.group0(),
            // e235, e315, e125, e5
            (other.group1() + self.group1().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<CircleOrthogonalOrigin> for VersorEvenOrthogonalOrigin {
    fn add_assign(&mut self, other: CircleOrthogonalOrigin) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            other.group0() + self.group0(),
            // e235, e315, e125, e5
            (other.group1() + self.group1().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Add<CircleRotor> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn add(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() + self.group0().xyz()).with_w(other[e12345]),
            // e415, e425, e435, e321
            other.group1() + Simd32x3::from(0.0).with_w(self[e321]),
            // e235, e315, e125, e5
            (other.group2().xyz() + self.group1().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Add<CircleRotorAligningOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() + self.group0().xyz()).with_w(other[e12345]),
            // e415, e425, e435, e321
            other.group1().with_w(self[e321]),
            // e235, e315, e125, e5
            (other.group2().xyz() + self.group1().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Add<CircleRotorAligningOriginAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]),
            // e415, e425, e435, e321
            other.group0().with_w(self[e321]),
            // e235, e315, e125, e5
            (other.group1().xyz() + self.group1().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Add<CircleRotorAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: CircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]),
            // e415, e425, e435, e321
            other.group0() + Simd32x3::from(0.0).with_w(self[e321]),
            // e235, e315, e125, e5
            (other.group1().xyz() + self.group1().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Add<CircleRotorOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: CircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() + self.group0().xyz().with_w(0.0),
            // e415, e425, e435, e321
            other.group1().with_w(self[e321]),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Add<Dipole> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(other[e45]),
            // e15, e25, e35
            other.group2(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleAligningOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(0.0),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleInversion> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(other[e45]),
            // e15, e25, e35
            other.group2().xyz(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionAligningOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            other.group1().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().xyz().with_w(0.0),
            // e15, e25, e35
            other.group1().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            other.group1(),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleInversionOrthogonalOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().xyz().with_w(0.0),
            // e15, e25, e35
            other.group2().xyz(),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleOrthogonalOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(0.0),
            // e15, e25, e35
            other.group2(),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DualNum> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(other[e4] + self[e4]),
        );
    }
}
impl std::ops::Add<FlatOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<FlatPoint> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            other.group0().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<FlatPointAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: FlatPointAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group0(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Flector> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            other.group0().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<FlectorAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group0().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<FlectorOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Horizon> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<Infinity> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1() + Simd32x3::from(0.0).with_w(other[e5]),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<Infinity> for VersorEvenOrthogonalOrigin {
    fn add_assign(&mut self, other: Infinity) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1() + Simd32x3::from(0.0).with_w(other[e5]),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Add<Line> for VersorEvenOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            other.group0().with_w(self[e321]),
            // e235, e315, e125, e4
            (other.group1() + self.group1().xyz()).with_w(self[e4]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl std::ops::Add<LineAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: LineAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            (other.group0() + self.group1().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<LineAtInfinity> for VersorEvenOrthogonalOrigin {
    fn add_assign(&mut self, other: LineAtInfinity) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            (other.group0() + self.group1().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Add<LineOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    fn add(self, other: LineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            other.group0().with_w(self[e321]),
            // e235, e315, e125, e4
            Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl std::ops::Add<Motor> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]),
            // e415, e425, e435, e321
            other.group0().xyz().with_w(self[e321]),
            // e235, e315, e125, e5
            other.group1() + self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Add<MotorAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: MotorAtInfinity) -> Self::Output {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            other.group0() + self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<MotorAtInfinity> for VersorEvenOrthogonalOrigin {
    fn add_assign(&mut self, other: MotorAtInfinity) {
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            other.group0() + self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Add<MotorOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    fn add(self, other: MotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]),
            // e415, e425, e435, e321
            other.group0().xyz().with_w(self[e321]),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Add<MultiVector> for VersorEvenOrthogonalOrigin {
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
        return MultiVector::from_groups(
            // scalar, e12345
            other.group0(),
            // e1, e2, e3, e4
            other.group1() + self.group2(),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            other.group3(),
            // e15, e25, e35
            other.group4(),
            // e23, e31, e12
            other.group5(),
            // e415, e425, e435, e321
            other.group6() + Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            other.group7() + self.group0().xyz(),
            // e235, e315, e125
            other.group8() + self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            other.group9(),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<MysteryCircle> for VersorEvenOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: MysteryCircle) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            other.group0() + Simd32x3::from(0.0).with_w(self[e321]),
            // e235, e315, e125, e4
            Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl std::ops::Add<MysteryCircleRotor> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]),
            // e415, e425, e435, e321
            other.group0() + Simd32x3::from(0.0).with_w(self[e321]),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Add<MysteryDipole> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MysteryDipoleInversion> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MysteryVersorEven> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: MysteryVersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]),
            // e415, e425, e435, e321
            other.group1() + Simd32x3::from(0.0).with_w(self[e321]),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            (self.group2().xyz() + other.group0().yzw()).with_w(self[e4]),
        );
    }
}
impl std::ops::Add<MysteryVersorOdd> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullCircleAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: NullCircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            (other.group0() + self.group0().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<NullCircleAtOrigin> for VersorEvenOrthogonalOrigin {
    fn add_assign(&mut self, other: NullCircleAtOrigin) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            (other.group0() + self.group0().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Add<NullDipoleAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: NullDipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullDipoleInversionAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().xyz().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullSphereAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullVersorEvenAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            (other.group0().xyz() + self.group0().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(other[e4] + self[e4]),
        );
    }
}
impl std::ops::AddAssign<NullVersorEvenAtOrigin> for VersorEvenOrthogonalOrigin {
    fn add_assign(&mut self, other: NullVersorEvenAtOrigin) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            (other.group0().xyz() + self.group0().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(other[e4] + self[e4]),
        );
    }
}
impl std::ops::Add<Origin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2() + Simd32x3::from(0.0).with_w(other[e4]),
        );
    }
}
impl std::ops::AddAssign<Origin> for VersorEvenOrthogonalOrigin {
    fn add_assign(&mut self, other: Origin) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2() + Simd32x3::from(0.0).with_w(other[e4]),
        );
    }
}
impl std::ops::Add<Plane> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<PlaneOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<RoundPoint> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1() + Simd32x3::from(0.0).with_w(other[e5]),
            // e1, e2, e3, e4
            other.group0() + self.group2(),
        );
    }
}
impl std::ops::AddAssign<RoundPoint> for VersorEvenOrthogonalOrigin {
    fn add_assign(&mut self, other: RoundPoint) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1() + Simd32x3::from(0.0).with_w(other[e5]),
            // e1, e2, e3, e4
            other.group0() + self.group2(),
        );
    }
}
impl std::ops::Add<RoundPointAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1().xyz().with_w(other[e5] + self[e5]),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(other[e4] + self[e4]),
        );
    }
}
impl std::ops::AddAssign<RoundPointAtOrigin> for VersorEvenOrthogonalOrigin {
    fn add_assign(&mut self, other: RoundPointAtOrigin) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1().xyz().with_w(other[e5] + self[e5]),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(other[e4] + self[e4]),
        );
    }
}
impl std::ops::Add<Scalar> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Sphere> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<SphereAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<SphereOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            other.group0().wxyz(),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<VersorEven> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        4        0        0
    // no simd       16        0        0
    fn add(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() + self.group0().xyz().with_w(0.0),
            // e415, e425, e435, e321
            other.group1() + Simd32x3::from(0.0).with_w(self[e321]),
            // e235, e315, e125, e5
            other.group2() + self.group1(),
            // e1, e2, e3, e4
            other.group3() + self.group2(),
        );
    }
}
impl std::ops::Add<VersorEvenAligningOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        9        0        0
    fn add(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() + self.group0().xyz().with_w(0.0),
            // e415, e425, e435, e321
            other.group1().xyz().with_w(self[e321]),
            // e235, e315, e125, e5
            other.group2() + self.group1(),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(other[e4] + self[e4]),
        );
    }
}
impl std::ops::Add<VersorEvenAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       11        0        0
    fn add(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]),
            // e415, e425, e435, e321
            other.group1() + Simd32x3::from(0.0).with_w(self[e321]),
            // e235, e315, e125, e5
            other.group2() + self.group1(),
            // e1, e2, e3, e4
            (self.group2().xyz() + other.group0().yzw()).with_w(self[e4]),
        );
    }
}
impl std::ops::Add<VersorEvenAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        8        0        0
    fn add(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            (other.group0().xyz() + self.group0().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            other.group1() + self.group1(),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(other[e4] + self[e4]),
        );
    }
}
impl std::ops::AddAssign<VersorEvenAtOrigin> for VersorEvenOrthogonalOrigin {
    fn add_assign(&mut self, other: VersorEvenAtOrigin) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            (other.group0().xyz() + self.group0().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            other.group1() + self.group1(),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(other[e4] + self[e4]),
        );
    }
}
impl std::ops::Add<VersorEvenOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: VersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() + self.group0().xyz().with_w(0.0),
            // e415, e425, e435, e321
            other.group1().xyz().with_w(self[e321]),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(other[e4] + self[e4]),
        );
    }
}
impl std::ops::Add<VersorEvenOrthogonalOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        3        0        0
    // no simd       12        0        0
    fn add(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            other.group0() + self.group0(),
            // e235, e315, e125, e5
            other.group1() + self.group1(),
            // e1, e2, e3, e4
            other.group2() + self.group2(),
        );
    }
}
impl std::ops::AddAssign<VersorEvenOrthogonalOrigin> for VersorEvenOrthogonalOrigin {
    fn add_assign(&mut self, other: VersorEvenOrthogonalOrigin) {
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            other.group0() + self.group0(),
            // e235, e315, e125, e5
            other.group1() + self.group1(),
            // e1, e2, e3, e4
            other.group2() + self.group2(),
        );
    }
}
impl std::ops::Add<VersorOdd> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], other[e45]]),
            // e15, e25, e35
            other.group2().xyz(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<VersorOddAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            other.group0().yzw(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<VersorOddOrthogonalOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().xyz().with_w(0.0),
            // e15, e25, e35
            other.group2().xyz(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            other[e3215],
        );
    }
}

impl From<AntiDipoleInversionOnOrigin> for VersorEvenOrthogonalOrigin {
    fn from(from_anti_dipole_inversion_on_origin: AntiDipoleInversionOnOrigin) -> Self {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            from_anti_dipole_inversion_on_origin.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            from_anti_dipole_inversion_on_origin.group1().yzwx(),
        );
    }
}

impl From<AntiDipoleOnOrigin> for VersorEvenOrthogonalOrigin {
    fn from(from_anti_dipole_on_origin: AntiDipoleOnOrigin) -> Self {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            from_anti_dipole_on_origin.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiFlatOrigin> for VersorEvenOrthogonalOrigin {
    fn from(from_anti_flat_origin: AntiFlatOrigin) -> Self {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(from_anti_flat_origin[e321]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiFlatPoint> for VersorEvenOrthogonalOrigin {
    fn from(from_anti_flat_point: AntiFlatPoint) -> Self {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(from_anti_flat_point[e321]),
            // e235, e315, e125, e5
            from_anti_flat_point.group0().xyz().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiFlector> for VersorEvenOrthogonalOrigin {
    fn from(from_anti_flector: AntiFlector) -> Self {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(from_anti_flector[e321]),
            // e235, e315, e125, e5
            Simd32x4::from([from_anti_flector[e235], from_anti_flector[e315], from_anti_flector[e125], from_anti_flector[e5]]),
            // e1, e2, e3, e4
            from_anti_flector.group1().xyz().with_w(0.0),
        );
    }
}

impl From<AntiFlectorOnOrigin> for VersorEvenOrthogonalOrigin {
    fn from(from_anti_flector_on_origin: AntiFlectorOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(from_anti_flector_on_origin[e321]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([from_anti_flector_on_origin[e1], from_anti_flector_on_origin[e2], from_anti_flector_on_origin[e3], 0.0]),
        );
    }
}

impl From<AntiPlane> for VersorEvenOrthogonalOrigin {
    fn from(from_anti_plane: AntiPlane) -> Self {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(from_anti_plane[e5]),
            // e1, e2, e3, e4
            from_anti_plane.group0().xyz().with_w(0.0),
        );
    }
}

impl From<AntiPlaneOnOrigin> for VersorEvenOrthogonalOrigin {
    fn from(from_anti_plane_on_origin: AntiPlaneOnOrigin) -> Self {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            from_anti_plane_on_origin.group0().with_w(0.0),
        );
    }
}

impl From<AntiSphereOnOrigin> for VersorEvenOrthogonalOrigin {
    fn from(from_anti_sphere_on_origin: AntiSphereOnOrigin) -> Self {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            from_anti_sphere_on_origin.group0(),
        );
    }
}

impl From<CircleAtOrigin> for VersorEvenOrthogonalOrigin {
    fn from(from_circle_at_origin: CircleAtOrigin) -> Self {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            from_circle_at_origin.group0().with_w(0.0),
            // e235, e315, e125, e5
            from_circle_at_origin.group1().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleOrthogonalOrigin> for VersorEvenOrthogonalOrigin {
    fn from(from_circle_orthogonal_origin: CircleOrthogonalOrigin) -> Self {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            from_circle_orthogonal_origin.group0(),
            // e235, e315, e125, e5
            from_circle_orthogonal_origin.group1().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<Infinity> for VersorEvenOrthogonalOrigin {
    fn from(from_infinity: Infinity) -> Self {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(from_infinity[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<LineAtInfinity> for VersorEvenOrthogonalOrigin {
    fn from(from_line_at_infinity: LineAtInfinity) -> Self {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            from_line_at_infinity.group0().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<MotorAtInfinity> for VersorEvenOrthogonalOrigin {
    fn from(from_motor_at_infinity: MotorAtInfinity) -> Self {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            from_motor_at_infinity.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<NullCircleAtOrigin> for VersorEvenOrthogonalOrigin {
    fn from(from_null_circle_at_origin: NullCircleAtOrigin) -> Self {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            from_null_circle_at_origin.group0().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<NullVersorEvenAtOrigin> for VersorEvenOrthogonalOrigin {
    fn from(from_null_versor_even_at_origin: NullVersorEvenAtOrigin) -> Self {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            from_null_versor_even_at_origin.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_null_versor_even_at_origin[e4]),
        );
    }
}

impl From<Origin> for VersorEvenOrthogonalOrigin {
    fn from(from_origin: Origin) -> Self {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_origin[e4]),
        );
    }
}

impl From<RoundPoint> for VersorEvenOrthogonalOrigin {
    fn from(from_round_point: RoundPoint) -> Self {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(from_round_point[e5]),
            // e1, e2, e3, e4
            from_round_point.group0(),
        );
    }
}

impl From<RoundPointAtOrigin> for VersorEvenOrthogonalOrigin {
    fn from(from_round_point_at_origin: RoundPointAtOrigin) -> Self {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(from_round_point_at_origin[e5]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_round_point_at_origin[e4]),
        );
    }
}

impl From<VersorEvenAtOrigin> for VersorEvenOrthogonalOrigin {
    fn from(from_versor_even_at_origin: VersorEvenAtOrigin) -> Self {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            from_versor_even_at_origin.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            from_versor_even_at_origin.group1(),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_versor_even_at_origin[e4]),
        );
    }
}
impl std::ops::Mul<AntiCircleOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       42        0
    //    simd3        0        5        0
    //    simd4        9        4        0
    // Totals...
    // yes simd       32       51        0
    //  no simd       59       73        0
    fn mul(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       49        0
    //    simd3        0        9        0
    //    simd4       22       14        0
    // Totals...
    // yes simd       53       72        0
    //  no simd      119      132        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       49        0
    //    simd3        0        9        0
    //    simd4       19       11        0
    // Totals...
    // yes simd       50       69        0
    //  no simd      107      120        0
    fn mul(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOriginAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       37        0
    //    simd3        0        4        0
    //    simd4       12        9        0
    // Totals...
    // yes simd       32       50        0
    //  no simd       68       85        0
    fn mul(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       36        0
    //    simd3        0        3        0
    //    simd4       15       13        0
    // Totals...
    // yes simd       35       52        0
    //  no simd       80       97        0
    fn mul(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       46        0
    //    simd3        0        5        0
    //    simd4       11        6        0
    // Totals...
    // yes simd       38       57        0
    //  no simd       71       85        0
    fn mul(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       50        0
    //    simd3        0       15        0
    //    simd4       37       22        0
    // Totals...
    // yes simd       53       87        0
    //  no simd      164      183        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       35        0
    //    simd3        0        8        0
    //    simd4       27       19        0
    // Totals...
    // yes simd       40       62        0
    //  no simd      121      135        0
    fn mul(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       39        0
    //    simd3        0        3        0
    //    simd4       15       12        0
    // Totals...
    // yes simd       38       54        0
    //  no simd       83       96        0
    fn mul(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOrthogonalOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       46        0
    //    simd3        0       10        0
    //    simd4       24       14        0
    // Totals...
    // yes simd       48       70        0
    //  no simd      120      132        0
    fn mul(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       24        0
    //    simd3        2        5        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       16       33        0
    //  no simd       32       55        0
    fn mul(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1       10        0
    //    simd3        1        2        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        3       15        0
    //  no simd        8       28        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        6        0
    // no simd        0       24        0
    fn mul(self, other: AntiFlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       17        0
    //    simd3        3        5        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       15       26        0
    //  no simd       33       48        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       35        0
    //    simd3        0        3        0
    //    simd4       16       13        0
    // Totals...
    // yes simd       40       51        0
    //  no simd       88       96        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlectorOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        8       10        0
    // Totals...
    // yes simd       12       18        0
    //  no simd       36       48        0
    fn mul(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       37        0
    //    simd3        0        4        0
    //    simd4        9        6        0
    // Totals...
    // yes simd       29       47        0
    //  no simd       56       73        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLineOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       26        0
    //    simd3        0        2        0
    //    simd4        3        1        0
    // Totals...
    // yes simd       15       29        0
    //  no simd       24       36        0
    fn mul(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiLineOnOrigin> for VersorEvenOrthogonalOrigin {
    fn mul_assign(&mut self, other: AntiLineOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       36        0
    //    simd3        0        3        0
    //    simd4       15       13        0
    // Totals...
    // yes simd       35       52        0
    //  no simd       80       97        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotorOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       18       30        0
    //  no simd       36       48        0
    fn mul(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMotorOnOrigin> for VersorEvenOrthogonalOrigin {
    fn mul_assign(&mut self, other: AntiMotorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryCircleRotor> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       20        0
    //    simd3        4        8        0
    //    simd4        6        5        0
    // Totals...
    // yes simd       18       33        0
    //  no simd       44       64        0
    fn mul(self, other: AntiMysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryDipoleInversion> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       29        0
    //    simd3        0        5        0
    //    simd4       15       10        0
    // Totals...
    // yes simd       28       44        0
    //  no simd       73       84        0
    fn mul(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       14        0
    //    simd3        3        5        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       14       24        0
    //  no simd       35       49        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlaneOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       23        0
    //    simd3        0        3        0
    //    simd4        5        2        0
    // Totals...
    // yes simd        9       28        0
    //  no simd       24       40        0
    fn mul(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for VersorEvenOrthogonalOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        6        0
    // no simd        0       24        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiSphereOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       17        0
    //    simd3        3        6        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       11       28        0
    //  no simd       32       55        0
    fn mul(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorEvenOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       40        0
    //    simd3        0        3        0
    //    simd4       15       12        0
    // Totals...
    // yes simd       38       55        0
    //  no simd       83       97        0
    fn mul(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       46        0
    //    simd3        0       10        0
    //    simd4       21       11        0
    // Totals...
    // yes simd       45       67        0
    //  no simd      108      120        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAligningOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       48        0
    //    simd3        0        8        0
    //    simd4       17        9        0
    // Totals...
    // yes simd       45       65        0
    //  no simd       96      108        0
    fn mul(self, other: CircleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       33        0
    //    simd3        0        2        0
    //    simd4       14       12        0
    // Totals...
    // yes simd       26       47        0
    //  no simd       68       87        0
    fn mul(self, other: CircleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       41        0
    //    simd3        0        7        0
    //    simd4       11        4        0
    // Totals...
    // yes simd       24       52        0
    //  no simd       57       78        0
    fn mul(self, other: CircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       41        0
    //    simd3        0        5        0
    //    simd4        9        4        0
    // Totals...
    // yes simd       33       50        0
    //  no simd       60       72        0
    fn mul(self, other: CircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOrthogonalOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       42        0
    //    simd3        0        6        0
    //    simd4       13        7        0
    // Totals...
    // yes simd       29       55        0
    //  no simd       68       88        0
    fn mul(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       54        0
    //    simd3        0        9        0
    //    simd4       22       13        0
    // Totals...
    // yes simd       53       76        0
    //  no simd      119      133        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       53        0
    //    simd3        0        9        0
    //    simd4       19       10        0
    // Totals...
    // yes simd       51       72        0
    //  no simd      108      120        0
    fn mul(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOriginAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       34        0
    //    simd3        0        2        0
    //    simd4       13       11        0
    // Totals...
    // yes simd       33       47        0
    //  no simd       72       84        0
    fn mul(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       33        0
    //    simd3        0        1        0
    //    simd4       16       15        0
    // Totals...
    // yes simd       35       49        0
    //  no simd       83       96        0
    fn mul(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       46        0
    //    simd3        0        6        0
    //    simd4       11        5        0
    // Totals...
    // yes simd       39       57        0
    //  no simd       72       84        0
    fn mul(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       50        0
    //    simd3        0       10        0
    //    simd4       19       10        0
    // Totals...
    // yes simd       47       70        0
    //  no simd      104      120        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAligningOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       36        0
    //    simd3        0        4        0
    //    simd4       13        9        0
    // Totals...
    // yes simd       31       49        0
    //  no simd       70       84        0
    fn mul(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       36        0
    //    simd3        0        3        0
    //    simd4       12       10        0
    // Totals...
    // yes simd       32       49        0
    //  no simd       68       85        0
    fn mul(self, other: DipoleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       38        0
    //    simd3        0        5        0
    //    simd4       10        5        0
    // Totals...
    // yes simd       27       48        0
    //  no simd       57       73        0
    fn mul(self, other: DipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       60        0
    //    simd3        0       12        0
    //    simd4       33       21        0
    // Totals...
    // yes simd       65       93        0
    //  no simd      164      180        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAligningOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       51        0
    //    simd3        0        7        0
    //    simd4       25       18        0
    // Totals...
    // yes simd       54       76        0
    //  no simd      129      144        0
    fn mul(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       44        0
    //    simd3        0        4        0
    //    simd4       23       19        0
    // Totals...
    // yes simd       47       67        0
    //  no simd      116      132        0
    fn mul(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       39        0
    //    simd3        0        3        0
    //    simd4       15       12        0
    // Totals...
    // yes simd       36       54        0
    //  no simd       81       96        0
    fn mul(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       35        0
    //    simd3        0        3        0
    //    simd4       16       13        0
    // Totals...
    // yes simd       36       51        0
    //  no simd       84       96        0
    fn mul(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOrthogonalOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       52        0
    //    simd3        0       11        0
    //    simd4       22       12        0
    // Totals...
    // yes simd       50       75        0
    //  no simd      116      133        0
    fn mul(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       21        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       17       29        0
    //  no simd       35       53        0
    fn mul(self, other: DipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOrthogonalOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       48        0
    //    simd3        0        8        0
    //    simd4       16        9        0
    // Totals...
    // yes simd       44       65        0
    //  no simd       92      108        0
    fn mul(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        8        0
    //    simd3        2        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        4       16        0
    //  no simd        8       36        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        6        0
    // no simd        0       24        0
    fn mul(self, other: FlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       21        0
    //    simd3        2        4        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       16       29        0
    //  no simd       32       49        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPointAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       15       27        0
    //  no simd       24       36        0
    fn mul(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       42        0
    //    simd3        0        2        0
    //    simd4       14       12        0
    // Totals...
    // yes simd       42       56        0
    //  no simd       84       96        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       18       30        0
    //  no simd       36       48        0
    fn mul(self, other: FlectorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        8       10        0
    // Totals...
    // yes simd       12       18        0
    //  no simd       36       48        0
    fn mul(self, other: FlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn mul(self, other: Horizon) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Infinity> for VersorEvenOrthogonalOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        5        0
    // no simd        0       20        0
    fn mul(self, other: Infinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       31        0
    //    simd3        0        2        0
    //    simd4       11        9        0
    // Totals...
    // yes simd       24       42        0
    //  no simd       57       73        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       22        0
    //    simd3        0        2        0
    //    simd4        5        3        0
    // Totals...
    // yes simd        9       27        0
    //  no simd       24       40        0
    fn mul(self, other: LineAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       21        0
    //    simd3        0        1        0
    //    simd4        5        4        0
    // Totals...
    // yes simd        9       26        0
    //  no simd       24       40        0
    fn mul(self, other: LineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       33        0
    //    simd3        0        1        0
    //    simd4       16       15        0
    // Totals...
    // yes simd       36       49        0
    //  no simd       84       96        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        8       10        0
    // Totals...
    // yes simd       12       18        0
    //  no simd       36       48        0
    fn mul(self, other: MotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        8       10        0
    // Totals...
    // yes simd       12       18        0
    //  no simd       36       48        0
    fn mul(self, other: MotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       81        0
    //    simd2        6        6        0
    //    simd3       44       57        0
    //    simd4       38       30        0
    // Totals...
    // yes simd      144      174        0
    //  no simd      352      384        0
    fn mul(self, other: MultiVector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircle> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       18        0
    //    simd3        3        6        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       17       28        0
    //  no simd       35       52        0
    fn mul(self, other: MysteryCircle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircleRotor> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       22        0
    //    simd3        4        9        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       21       35        0
    //  no simd       47       65        0
    fn mul(self, other: MysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipole> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        0
    //    simd3        3        6        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       14       28        0
    //  no simd       32       52        0
    fn mul(self, other: MysteryDipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipoleInversion> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       21        0
    //    simd3        0        1        0
    //    simd4       15       15        0
    // Totals...
    // yes simd       25       37        0
    //  no simd       70       84        0
    fn mul(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorEven> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd3        0        4        0
    //    simd4       17       14        0
    // Totals...
    // yes simd       33       46        0
    //  no simd       84       96        0
    fn mul(self, other: MysteryVersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorOdd> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       21        0
    //    simd3        0        1        0
    //    simd4       17       18        0
    // Totals...
    // yes simd       30       40        0
    //  no simd       81       96        0
    fn mul(self, other: MysteryVersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullCircleAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       20        0
    //    simd3        0        4        0
    //    simd4        4        1        0
    // Totals...
    // yes simd       14       25        0
    //  no simd       26       36        0
    fn mul(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       23        0
    //    simd3        0        3        0
    //    simd4        5        2        0
    // Totals...
    // yes simd        9       28        0
    //  no simd       24       40        0
    fn mul(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleInversionAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        8       10        0
    // Totals...
    // yes simd       12       18        0
    //  no simd       36       48        0
    fn mul(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullSphereAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        5        0
    // no simd        0       20        0
    fn mul(self, other: NullSphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullVersorEvenAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       20        0
    //    simd3        0        4        0
    //    simd4        7        4        0
    // Totals...
    // yes simd       18       28        0
    //  no simd       39       48        0
    fn mul(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Origin> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       27        0
    fn mul(self, other: Origin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       20        0
    //    simd3        3        5        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       14       29        0
    //  no simd       32       51        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<PlaneOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       20        0
    //    simd4        5        5        0
    // Totals...
    // yes simd        9       25        0
    //  no simd       24       40        0
    fn mul(self, other: PlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       30        0
    //    simd3        0        3        0
    //    simd4        9        6        0
    // Totals...
    // yes simd       21       39        0
    //  no simd       48       63        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPointAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1       10        0
    //    simd3        1        2        0
    //    simd4        1        5        0
    // Totals...
    // yes simd        3       17        0
    //  no simd        8       36        0
    fn mul(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn mul(self, other: Scalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for VersorEvenOrthogonalOrigin {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       30        0
    //    simd3        0        3        0
    //    simd4        9        6        0
    // Totals...
    // yes simd       21       39        0
    //  no simd       48       63        0
    fn mul(self, other: Sphere) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        8        0
    //    simd3        1        2        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        3       14        0
    //  no simd        8       30        0
    fn mul(self, other: SphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       21        0
    //    simd3        3        6        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       14       31        0
    //  no simd       32       55        0
    fn mul(self, other: SphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       49        0
    //    simd3        0       17        0
    //    simd4       39       23        0
    // Totals...
    // yes simd       59       89        0
    //  no simd      176      192        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAligningOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       49        0
    //    simd3        0        9        0
    //    simd4       26       17        0
    // Totals...
    // yes simd       54       75        0
    //  no simd      132      144        0
    fn mul(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       31        0
    //    simd3        0        7        0
    //    simd4       29       23        0
    // Totals...
    // yes simd       45       61        0
    //  no simd      132      144        0
    fn mul(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       39        0
    //    simd3        0        8        0
    //    simd4       16        9        0
    // Totals...
    // yes simd       33       56        0
    //  no simd       81       99        0
    fn mul(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       48        0
    //    simd3        0        5        0
    //    simd4       14        9        0
    // Totals...
    // yes simd       42       62        0
    //  no simd       84       99        0
    fn mul(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOrthogonalOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       39        0
    //    simd3        0       12        0
    //    simd4       28       18        0
    // Totals...
    // yes simd       44       69        0
    //  no simd      128      147        0
    fn mul(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       53        0
    //    simd3        0        5        0
    //    simd4       36       31        0
    // Totals...
    // yes simd       68       89        0
    //  no simd      176      192        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       41        0
    //    simd3        0        1        0
    //    simd4       26       25        0
    // Totals...
    // yes simd       50       67        0
    //  no simd      128      144        0
    fn mul(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddOrthogonalOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       45        0
    //    simd3        0        4        0
    //    simd4       25       22        0
    // Totals...
    // yes simd       53       71        0
    //  no simd      128      145        0
    fn mul(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Neg for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn neg(self) -> Self::Output {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from(-1.0),
            // e1, e2, e3, e4
            self.group2() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Not for VersorEvenOrthogonalOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiCircleOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: AntiCircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotor> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(other[e45]) * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       11        0
    fn sub(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOriginAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        8        0
    fn sub(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn sub(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        8        0
    fn sub(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiDipoleInversion> for VersorEvenOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        4        1        0
    //  no simd       12        4        0
    fn sub(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz() - other.group0(),
            // e415, e425, e435, e321
            other.group1().xyz().with_w(self[e321] - other[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            self.group1().xyz().with_w(self[e4]) - other.group2(),
            // e1, e2, e3, e5
            self.group2().xyz().with_w(self[e5]) - other.group3(),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        9        7        0
    fn sub(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            other.group0().xyz().with_w(self[e321] - other[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group1().xyz().with_w(self[e4]),
            // e1, e2, e3, e5
            self.group2().xyz().with_w(self[e5]) - other.group2(),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn sub(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() - other.group0(),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2() - other.group1().yzwx(),
        );
    }
}
impl std::ops::SubAssign<AntiDipoleInversionOnOrigin> for VersorEvenOrthogonalOrigin {
    fn sub_assign(&mut self, other: AntiDipoleInversionOnOrigin) {
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() - other.group0(),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2() - other.group1().yzwx(),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOrthogonalOrigin> for VersorEvenOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        8        4        0
    fn sub(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz() - other.group0().xyz(),
            // e415, e425, e435, e321
            other.group1().with_w(self[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            self.group1().xyz().with_w(self[e4]) - other.group2(),
            // e1, e2, e3, e5
            self.group2().xyz().with_w(self[e5] - other[e5]),
        );
    }
}
impl std::ops::Sub<AntiDipoleOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() - other.group0(),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiDipoleOnOrigin> for VersorEvenOrthogonalOrigin {
    fn sub_assign(&mut self, other: AntiDipoleOnOrigin) {
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() - other.group0(),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiDualNum> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        2        0
    // no simd        0        4        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiFlatOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiFlatOrigin> for VersorEvenOrthogonalOrigin {
    fn sub_assign(&mut self, other: AntiFlatOrigin) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiFlatPoint> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        4        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiFlatPoint> for VersorEvenOrthogonalOrigin {
    fn sub_assign(&mut self, other: AntiFlatPoint) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiFlector> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        3        4        0
    //  no simd       12        4        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125, e5
            self.group1() - other.group0().xyz().with_w(other[e5]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiFlector> for VersorEvenOrthogonalOrigin {
    fn sub_assign(&mut self, other: AntiFlector) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125, e5
            self.group1() - other.group0().xyz().with_w(other[e5]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<AntiFlectorOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        4        0
    fn sub(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiFlectorOnOrigin> for VersorEvenOrthogonalOrigin {
    fn sub_assign(&mut self, other: AntiFlectorOnOrigin) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<AntiLine> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiLineOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: AntiLineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiMotor> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        9        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<AntiMotorOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiMysteryCircleRotor> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn sub(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiMysteryDipoleInversion> for VersorEvenOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        5        7        0
    fn sub(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            other.group0().xyz().with_w(self[e321] - other[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group2().xyz().with_w(self[e5]),
        );
    }
}
impl std::ops::Sub<AntiPlane> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        4        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1() + Simd32x3::from(0.0).with_w(other[e5] * -1.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiPlane> for VersorEvenOrthogonalOrigin {
    fn sub_assign(&mut self, other: AntiPlane) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1() + Simd32x3::from(0.0).with_w(other[e5] * -1.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<AntiPlaneOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        4        3        0
    fn sub(self, other: AntiPlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiPlaneOnOrigin> for VersorEvenOrthogonalOrigin {
    fn sub_assign(&mut self, other: AntiPlaneOnOrigin) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<AntiScalar> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiSphereOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: AntiSphereOnOrigin) -> Self::Output {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2() - other.group0(),
        );
    }
}
impl std::ops::SubAssign<AntiSphereOnOrigin> for VersorEvenOrthogonalOrigin {
    fn sub_assign(&mut self, other: AntiSphereOnOrigin) {
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2() - other.group0(),
        );
    }
}
impl std::ops::Sub<AntiVersorEvenOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        2        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn sub(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Circle> for VersorEvenOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        1        0        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        8        7        0
    fn sub(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz() - other.group0(),
            // e415, e425, e435, e321
            other.group1().xyz().with_w(self[e321] - other[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group1().xyz().with_w(self[e4]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl std::ops::Sub<CircleAligningOrigin> for VersorEvenOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        1        0        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        7        7        0
    fn sub(self, other: CircleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz() - other.group0(),
            // e415, e425, e435, e321
            other.group1().with_w(self[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group1().xyz().with_w(self[e4]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl std::ops::Sub<CircleAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        5        7        0
    fn sub(self, other: CircleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            other.group0().xyz().with_w(self[e321] - other[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group1().xyz().with_w(self[e4]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl std::ops::Sub<CircleAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        8        6        0
    fn sub(self, other: CircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<CircleAtOrigin> for VersorEvenOrthogonalOrigin {
    fn sub_assign(&mut self, other: CircleAtOrigin) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Sub<CircleOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        3        4        0
    fn sub(self, other: CircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz() - other.group0(),
            // e415, e425, e435, e321
            other.group1().with_w(self[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl std::ops::Sub<CircleOrthogonalOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        8        3        0
    fn sub(self, other: CircleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() - other.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<CircleOrthogonalOrigin> for VersorEvenOrthogonalOrigin {
    fn sub_assign(&mut self, other: CircleOrthogonalOrigin) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() - other.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Sub<CircleRotor> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        1        0        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        8       11        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0().xyz() - other.group0()).with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            other.group1().xyz().with_w(self[e321] - other[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        1        0        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        7       11        0
    fn sub(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0().xyz() - other.group0()).with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            other.group1().with_w(self[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOriginAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        1        5        0
    //  no simd        4       11        0
    fn sub(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            other.group0().with_w(self[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Sub<CircleRotorAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        5       11        0
    fn sub(self, other: CircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            other.group0().xyz().with_w(self[e321] - other[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Sub<CircleRotorOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        3        8        0
    fn sub(self, other: CircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0().xyz() - other.group0().xyz()).with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            other.group1().with_w(self[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Sub<Dipole> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn sub(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(other[e45]) * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group2() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleAligningOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn sub(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleInversion> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       15        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(other[e45]) * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionAligningOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn sub(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       15        0
    fn sub(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        9        0
    fn sub(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            other.group1() * Simd32x4::from(-1.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionOrthogonalOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       12        0
    fn sub(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleOrthogonalOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn sub(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            other.group2() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DualNum> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        4        5        0
    fn sub(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::Sub<FlatOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<FlatPoint> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<FlatPointAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: FlatPointAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Flector> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn sub(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<FlectorAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<FlectorOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Horizon> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<Infinity> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1() + Simd32x3::from(0.0).with_w(other[e5] * -1.0),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<Infinity> for VersorEvenOrthogonalOrigin {
    fn sub_assign(&mut self, other: Infinity) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1() + Simd32x3::from(0.0).with_w(other[e5] * -1.0),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Sub<Line> for VersorEvenOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        4        7        0
    fn sub(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            other.group0().with_w(self[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group1().xyz().with_w(self[e4]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl std::ops::Sub<LineAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        4        3        0
    fn sub(self, other: LineAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<LineAtInfinity> for VersorEvenOrthogonalOrigin {
    fn sub_assign(&mut self, other: LineAtInfinity) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Sub<LineOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: LineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            other.group0().with_w(self[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl std::ops::Sub<Motor> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        2        0
    // no simd        4        8        0
    fn sub(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            other.group0().xyz().with_w(self[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1() - other.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Sub<MotorAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: MotorAtInfinity) -> Self::Output {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1() - other.group0(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<MotorAtInfinity> for VersorEvenOrthogonalOrigin {
    fn sub_assign(&mut self, other: MotorAtInfinity) {
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1() - other.group0(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Sub<MotorOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: MotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            other.group0().xyz().with_w(self[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Sub<MultiVector> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        1        0
    //    simd2        0        1        0
    //    simd3        2        2        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        5        7        0
    //  no simd       12       21        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            other.group0() * Simd32x2::from(-1.0),
            // e1, e2, e3, e4
            self.group2() - other.group1(),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            other.group3() * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group4() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group5() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group6().xyz().with_w(self[e321] - other[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            self.group0().xyz() - other.group7(),
            // e235, e315, e125
            self.group1().xyz() - other.group8(),
            // e1234, e4235, e4315, e4125
            other.group9() * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<MysteryCircle> for VersorEvenOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        1        4        0
    fn sub(self, other: MysteryCircle) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            other.group0().xyz().with_w(self[e321] - other[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl std::ops::Sub<MysteryCircleRotor> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        1        8        0
    fn sub(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            other.group0().xyz().with_w(self[e321] - other[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Sub<MysteryDipole> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MysteryDipoleInversion> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn sub(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MysteryVersorEven> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        5       11        0
    fn sub(self, other: MysteryVersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            other.group1().xyz().with_w(self[e321] - other[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<MysteryVersorOdd> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       13        0
    fn sub(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullCircleAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        4        3        0
    fn sub(self, other: NullCircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<NullCircleAtOrigin> for VersorEvenOrthogonalOrigin {
    fn sub_assign(&mut self, other: NullCircleAtOrigin) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Sub<NullDipoleAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: NullDipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullDipoleInversionAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullSphereAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn sub(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullVersorEvenAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        4        0
    fn sub(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::SubAssign<NullVersorEvenAtOrigin> for VersorEvenOrthogonalOrigin {
    fn sub_assign(&mut self, other: NullVersorEvenAtOrigin) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::Sub<Origin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::SubAssign<Origin> for VersorEvenOrthogonalOrigin {
    fn sub_assign(&mut self, other: Origin) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::Sub<Plane> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<PlaneOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<RoundPoint> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        8        1        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1() + Simd32x3::from(0.0).with_w(other[e5] * -1.0),
            // e1, e2, e3, e4
            self.group2() - other.group0(),
        );
    }
}
impl std::ops::SubAssign<RoundPoint> for VersorEvenOrthogonalOrigin {
    fn sub_assign(&mut self, other: RoundPoint) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1() + Simd32x3::from(0.0).with_w(other[e5] * -1.0),
            // e1, e2, e3, e4
            self.group2() - other.group0(),
        );
    }
}
impl std::ops::Sub<RoundPointAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        8        2        0
    fn sub(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1() + Simd32x3::from(0.0).with_w(other[e5] * -1.0),
            // e1, e2, e3, e4
            self.group2() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::SubAssign<RoundPointAtOrigin> for VersorEvenOrthogonalOrigin {
    fn sub_assign(&mut self, other: RoundPointAtOrigin) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1() + Simd32x3::from(0.0).with_w(other[e5] * -1.0),
            // e1, e2, e3, e4
            self.group2() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::Sub<Scalar> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Sphere> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<SphereAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn sub(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<SphereOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            other.group0().wxyz() * Simd32x4::from(-1.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<VersorEven> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        4        2        0
    //  no simd       12        8        0
    fn sub(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0().xyz() - other.group0().xyz()).with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            other.group1().xyz().with_w(self[e321] - other[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1() - other.group2(),
            // e1, e2, e3, e4
            self.group2() - other.group3(),
        );
    }
}
impl std::ops::Sub<VersorEvenAligningOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        0        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        3        3        0
    //  no simd       11        9        0
    fn sub(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0().xyz() - other.group0().xyz()).with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            other.group1().xyz().with_w(self[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1() - other.group2(),
            // e1, e2, e3, e4
            self.group2() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::Sub<VersorEvenAtInfinity> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        9       11        0
    fn sub(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            other.group1().xyz().with_w(self[e321] - other[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1() - other.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<VersorEvenAtOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        3        4        0
    //  no simd       12        4        0
    fn sub(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e235, e315, e125, e5
            self.group1() - other.group1(),
            // e1, e2, e3, e4
            self.group2() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::SubAssign<VersorEvenAtOrigin> for VersorEvenOrthogonalOrigin {
    fn sub_assign(&mut self, other: VersorEvenAtOrigin) {
        use crate::elements::*;
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e235, e315, e125, e5
            self.group1() - other.group1(),
            // e1, e2, e3, e4
            self.group2() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::Sub<VersorEvenOnOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        0        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        7        9        0
    fn sub(self, other: VersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0().xyz() - other.group0().xyz()).with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            other.group1().xyz().with_w(self[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1(),
            // e1, e2, e3, e4
            self.group2() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::Sub<VersorEvenOrthogonalOrigin> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        3        0        0
    // no simd       12        0        0
    fn sub(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() - other.group0(),
            // e235, e315, e125, e5
            self.group1() - other.group1(),
            // e1, e2, e3, e4
            self.group2() - other.group2(),
        );
    }
}
impl std::ops::SubAssign<VersorEvenOrthogonalOrigin> for VersorEvenOrthogonalOrigin {
    fn sub_assign(&mut self, other: VersorEvenOrthogonalOrigin) {
        *self = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() - other.group0(),
            // e235, e315, e125, e5
            self.group1() - other.group1(),
            // e1, e2, e3, e4
            self.group2() - other.group2(),
        );
    }
}
impl std::ops::Sub<VersorOdd> for VersorEvenOrthogonalOrigin {
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
    fn sub(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().xyz().with_w(other[e45]) * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<VersorOddAtInfinity> for VersorEvenOrthogonalOrigin {
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
    fn sub(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            other.group0().yzw() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<VersorOddOrthogonalOrigin> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        2        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       14        0
    fn sub(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            self.group2(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group1().xyz(),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}

impl TryFrom<AntiDipoleInversion> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(anti_dipole_inversion: AntiDipoleInversion) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
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
        if fail {
            let mut error = "Elements from AntiDipoleInversion do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            anti_dipole_inversion.group0().with_w(anti_dipole_inversion[e321]),
            // e235, e315, e125, e5
            Simd32x4::from([anti_dipole_inversion[e235], anti_dipole_inversion[e315], anti_dipole_inversion[e125], anti_dipole_inversion[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([anti_dipole_inversion[e1], anti_dipole_inversion[e2], anti_dipole_inversion[e3], anti_dipole_inversion[e4]]),
        ));
    }
}

impl TryFrom<AntiDipoleInversionAtInfinity> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(anti_dipole_inversion_at_infinity: AntiDipoleInversionAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversionAtInfinity do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(anti_dipole_inversion_at_infinity[e321]),
            // e235, e315, e125, e5
            anti_dipole_inversion_at_infinity.group1().with_w(anti_dipole_inversion_at_infinity[e5]),
            // e1, e2, e3, e4
            anti_dipole_inversion_at_infinity.group2().xyz().with_w(0.0),
        ));
    }
}

impl TryFrom<AntiDipoleInversionOrthogonalOrigin> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(anti_dipole_inversion_orthogonal_origin: AntiDipoleInversionOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion_orthogonal_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_orthogonal_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_orthogonal_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversionOrthogonalOrigin do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            anti_dipole_inversion_orthogonal_origin.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                anti_dipole_inversion_orthogonal_origin[e235],
                anti_dipole_inversion_orthogonal_origin[e315],
                anti_dipole_inversion_orthogonal_origin[e125],
                anti_dipole_inversion_orthogonal_origin[e5],
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_dipole_inversion_orthogonal_origin[e4]),
        ));
    }
}

impl TryFrom<AntiMysteryDipoleInversion> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(anti_mystery_dipole_inversion: AntiMysteryDipoleInversion) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_mystery_dipole_inversion[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_mystery_dipole_inversion[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_mystery_dipole_inversion[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMysteryDipoleInversion do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(anti_mystery_dipole_inversion[e321]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            anti_mystery_dipole_inversion.group1().with_w(0.0),
        ));
    }
}

impl TryFrom<Circle> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(circle: Circle) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
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
            let mut error = "Elements from Circle do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            circle.group0().with_w(circle[e321]),
            // e235, e315, e125, e5
            circle.group2().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleAligningOrigin> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(circle_aligning_origin: CircleAligningOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_aligning_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_aligning_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_aligning_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleAligningOrigin do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            circle_aligning_origin.group0().with_w(0.0),
            // e235, e315, e125, e5
            circle_aligning_origin.group2().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleAtInfinity> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(circle_at_infinity: CircleAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleAtInfinity do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(circle_at_infinity[e321]),
            // e235, e315, e125, e5
            circle_at_infinity.group1().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleOnOrigin> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(circle_on_origin: CircleOnOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_on_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_on_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleOnOrigin do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            circle_on_origin.group0().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleRotor> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(circle_rotor: CircleRotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
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
            let mut error = "Elements from CircleRotor do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            circle_rotor.group0().with_w(circle_rotor[e321]),
            // e235, e315, e125, e5
            circle_rotor.group2().xyz().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleRotorAligningOrigin> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(circle_rotor_aligning_origin: CircleRotorAligningOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor_aligning_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotorAligningOrigin do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            circle_rotor_aligning_origin.group0().with_w(0.0),
            // e235, e315, e125, e5
            circle_rotor_aligning_origin.group2().xyz().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleRotorAligningOriginAtInfinity> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(circle_rotor_aligning_origin_at_infinity: CircleRotorAligningOriginAtInfinity) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor_aligning_origin_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotorAligningOriginAtInfinity do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            circle_rotor_aligning_origin_at_infinity.group1().xyz().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleRotorAtInfinity> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(circle_rotor_at_infinity: CircleRotorAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_at_infinity[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotorAtInfinity do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(circle_rotor_at_infinity[e321]),
            // e235, e315, e125, e5
            circle_rotor_at_infinity.group1().xyz().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleRotorOnOrigin> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(circle_rotor_on_origin: CircleRotorOnOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_on_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_on_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_on_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotorOnOrigin do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            circle_rotor_on_origin.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<DualNum> for VersorEvenOrthogonalOrigin {
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
            let mut error = "Elements from DualNum do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(dual_num[e4]),
        ));
    }
}

impl TryFrom<Line> for VersorEvenOrthogonalOrigin {
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
            let mut error = "Elements from Line do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            line.group1().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<Motor> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(motor: Motor) -> Result<Self, Self::Error> {
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
            let mut error = "Elements from Motor do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            motor.group1(),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<MultiVector> for VersorEvenOrthogonalOrigin {
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
        let el = multi_vector[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
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
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
        let el = multi_vector[27];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[28];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[29];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[30];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[31];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MultiVector do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            multi_vector.group7().with_w(multi_vector[e321]),
            // e235, e315, e125, e5
            multi_vector.group8().with_w(multi_vector[e5]),
            // e1, e2, e3, e4
            multi_vector.group1(),
        ));
    }
}

impl TryFrom<MysteryCircle> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(mystery_circle: MysteryCircle) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_circle[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_circle[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_circle[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryCircle do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(mystery_circle[e321]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<MysteryCircleRotor> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(mystery_circle_rotor: MysteryCircleRotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_circle_rotor[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_circle_rotor[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_circle_rotor[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_circle_rotor[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryCircleRotor do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(mystery_circle_rotor[e321]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<MysteryVersorEven> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(mystery_versor_even: MysteryVersorEven) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_versor_even[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_even[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_even[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_even[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryVersorEven do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(mystery_versor_even[e321]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([mystery_versor_even[e1], mystery_versor_even[e2], mystery_versor_even[e3], 0.0]),
        ));
    }
}

impl TryFrom<VersorEven> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(versor_even: VersorEven) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
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
        if fail {
            let mut error = "Elements from VersorEven do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([versor_even[e423], versor_even[e431], versor_even[e412], versor_even[e321]]),
            // e235, e315, e125, e5
            versor_even.group2(),
            // e1, e2, e3, e4
            versor_even.group3(),
        ));
    }
}

impl TryFrom<VersorEvenAligningOrigin> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(versor_even_aligning_origin: VersorEvenAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_aligning_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAligningOrigin do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            versor_even_aligning_origin.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            versor_even_aligning_origin.group2(),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(versor_even_aligning_origin[e4]),
        ));
    }
}

impl TryFrom<VersorEvenAtInfinity> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(versor_even_at_infinity: VersorEvenAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAtInfinity do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(versor_even_at_infinity[e321]),
            // e235, e315, e125, e5
            versor_even_at_infinity.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([versor_even_at_infinity[e1], versor_even_at_infinity[e2], versor_even_at_infinity[e3], 0.0]),
        ));
    }
}

impl TryFrom<VersorEvenOnOrigin> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(versor_even_on_origin: VersorEvenOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_on_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_on_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_on_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenOnOrigin do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            versor_even_on_origin.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(versor_even_on_origin[e4]),
        ));
    }
}
