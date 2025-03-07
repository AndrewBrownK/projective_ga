use crate::traits::GeometricProduct;
use crate::traits::RightDual;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 334
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         1       2       0
//  Average:        11      17       0
//  Maximum:       190     221       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         4       3       0
//  Average:        28      33       0
//  Maximum:       480     512       0
impl std::ops::Add<AntiCircleOnOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiCircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotor> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(other[e45]),
            // e15, e25, e35
            other.group2().xyz(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(0.0),
            // e15, e25, e35
            other.group2().xyz(),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOriginAtInfinity> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1().xyz(),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorAtInfinity> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            other.group1().xyz(),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorOnOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().xyz().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiDipoleInversion> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       15        0        0
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() + self.group0().xyz()).with_w(self[e12345]),
            // e415, e425, e435, e321
            other.group1() + self.group1(),
            // e235, e315, e125, e5
            self.group2() + other.group2().xyz().with_w(other[e5]),
            // e1, e2, e3, e4
            self.group3() + other.group3().xyz().with_w(other[e4]),
        );
    }
}
impl std::ops::AddAssign<AntiDipoleInversion> for VersorEven {
    fn add_assign(&mut self, other: AntiDipoleInversion) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() + self.group0().xyz()).with_w(self[e12345]),
            // e415, e425, e435, e321
            other.group1() + self.group1(),
            // e235, e315, e125, e5
            self.group2() + other.group2().xyz().with_w(other[e5]),
            // e1, e2, e3, e4
            self.group3() + other.group3().xyz().with_w(other[e4]),
        );
    }
}
impl std::ops::Add<AntiDipoleInversionAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       11        0        0
    fn add(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            other.group0() + self.group1(),
            // e235, e315, e125, e5
            self.group2() + other.group1().with_w(other[e5]),
            // e1, e2, e3, e4
            (other.group2().xyz() + self.group3().xyz()).with_w(self[e4]),
        );
    }
}
impl std::ops::AddAssign<AntiDipoleInversionAtInfinity> for VersorEven {
    fn add_assign(&mut self, other: AntiDipoleInversionAtInfinity) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            other.group0() + self.group1(),
            // e235, e315, e125, e5
            self.group2() + other.group1().with_w(other[e5]),
            // e1, e2, e3, e4
            (other.group2().xyz() + self.group3().xyz()).with_w(self[e4]),
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        8        0        0
    fn add(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0().xyz() + self.group0().xyz()).with_w(self[e12345]),
            // e415, e425, e435, e321
            self.group1().xyz().with_w(other[e321] + self[e321]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3() + other.group1().yzwx(),
        );
    }
}
impl std::ops::AddAssign<AntiDipoleInversionOnOrigin> for VersorEven {
    fn add_assign(&mut self, other: AntiDipoleInversionOnOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0().xyz() + self.group0().xyz()).with_w(self[e12345]),
            // e415, e425, e435, e321
            self.group1().xyz().with_w(other[e321] + self[e321]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3() + other.group1().yzwx(),
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOrthogonalOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       11        0        0
    fn add(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0().xyz() + self.group0().xyz()).with_w(self[e12345]),
            // e415, e425, e435, e321
            (other.group1() + self.group1().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            self.group2() + other.group2().xyz().with_w(other[e5]),
            // e1, e2, e3, e4
            self.group3().xyz().with_w(other[e4] + self[e4]),
        );
    }
}
impl std::ops::AddAssign<AntiDipoleInversionOrthogonalOrigin> for VersorEven {
    fn add_assign(&mut self, other: AntiDipoleInversionOrthogonalOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0().xyz() + self.group0().xyz()).with_w(self[e12345]),
            // e415, e425, e435, e321
            (other.group1() + self.group1().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            self.group2() + other.group2().xyz().with_w(other[e5]),
            // e1, e2, e3, e4
            self.group3().xyz().with_w(other[e4] + self[e4]),
        );
    }
}
impl std::ops::Add<AntiDipoleOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: AntiDipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0().xyz() + self.group0().xyz()).with_w(self[e12345]),
            // e415, e425, e435, e321
            self.group1().xyz().with_w(other[e321] + self[e321]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<AntiDipoleOnOrigin> for VersorEven {
    fn add_assign(&mut self, other: AntiDipoleOnOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0().xyz() + self.group0().xyz()).with_w(self[e12345]),
            // e415, e425, e435, e321
            self.group1().xyz().with_w(other[e321] + self[e321]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<AntiDualNum> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiFlatOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1() + Simd32x3::from(0.0).with_w(other[e321]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<AntiFlatOrigin> for VersorEven {
    fn add_assign(&mut self, other: AntiFlatOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1() + Simd32x3::from(0.0).with_w(other[e321]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<AntiFlatPoint> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
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
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        8        0        0
    fn add(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
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
impl std::ops::Add<AntiFlectorOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1().xyz().with_w(other[e321] + self[e321]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            (self.group3().xyz() + other.group0().yzw()).with_w(self[e4]),
        );
    }
}
impl std::ops::AddAssign<AntiFlectorOnOrigin> for VersorEven {
    fn add_assign(&mut self, other: AntiFlectorOnOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1().xyz().with_w(other[e321] + self[e321]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            (self.group3().xyz() + other.group0().yzw()).with_w(self[e4]),
        );
    }
}
impl std::ops::Add<AntiLine> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiLineOnOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiLineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiMotor> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1().xyz(),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<AntiMotorOnOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiMysteryCircleRotor> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiMysteryDipoleInversion> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            other.group0() + self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            (other.group1() + self.group3().xyz()).with_w(self[e4]),
        );
    }
}
impl std::ops::AddAssign<AntiMysteryDipoleInversion> for VersorEven {
    fn add_assign(&mut self, other: AntiMysteryDipoleInversion) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            other.group0() + self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            (other.group1() + self.group3().xyz()).with_w(self[e4]),
        );
    }
}
impl std::ops::Add<AntiPlane> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
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
impl std::ops::Add<AntiPlaneOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: AntiPlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            (other.group0() + self.group3().xyz()).with_w(self[e4]),
        );
    }
}
impl std::ops::AddAssign<AntiPlaneOnOrigin> for VersorEven {
    fn add_assign(&mut self, other: AntiPlaneOnOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            (other.group0() + self.group3().xyz()).with_w(self[e4]),
        );
    }
}
impl std::ops::Add<AntiScalar> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + Simd32x3::from(0.0).with_w(other[e12345]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<AntiScalar> for VersorEven {
    fn add_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + Simd32x3::from(0.0).with_w(other[e12345]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<AntiSphereOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: AntiSphereOnOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            other.group0() + self.group3(),
        );
    }
}
impl std::ops::AddAssign<AntiSphereOnOrigin> for VersorEven {
    fn add_assign(&mut self, other: AntiSphereOnOrigin) {
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            other.group0() + self.group3(),
        );
    }
}
impl std::ops::Add<AntiVersorEvenOnOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().xyz().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Circle> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn add(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() + self.group0().xyz()).with_w(self[e12345]),
            // e415, e425, e435, e321
            other.group1() + self.group1(),
            // e235, e315, e125, e5
            (other.group2() + self.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<Circle> for VersorEven {
    fn add_assign(&mut self, other: Circle) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() + self.group0().xyz()).with_w(self[e12345]),
            // e415, e425, e435, e321
            other.group1() + self.group1(),
            // e235, e315, e125, e5
            (other.group2() + self.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<CircleAligningOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        3        0        0
    // no simd        9        0        0
    fn add(self, other: CircleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() + self.group0().xyz()).with_w(self[e12345]),
            // e415, e425, e435, e321
            (other.group1() + self.group1().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            (other.group2() + self.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<CircleAligningOrigin> for VersorEven {
    fn add_assign(&mut self, other: CircleAligningOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() + self.group0().xyz()).with_w(self[e12345]),
            // e415, e425, e435, e321
            (other.group1() + self.group1().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            (other.group2() + self.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<CircleAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: CircleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            other.group0() + self.group1(),
            // e235, e315, e125, e5
            (other.group1() + self.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<CircleAtInfinity> for VersorEven {
    fn add_assign(&mut self, other: CircleAtInfinity) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            other.group0() + self.group1(),
            // e235, e315, e125, e5
            (other.group1() + self.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<CircleAtOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: CircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() + self.group0().xyz()).with_w(self[e12345]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            (other.group1() + self.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<CircleAtOrigin> for VersorEven {
    fn add_assign(&mut self, other: CircleAtOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() + self.group0().xyz()).with_w(self[e12345]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            (other.group1() + self.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<CircleOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: CircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() + self.group0().xyz()).with_w(self[e12345]),
            // e415, e425, e435, e321
            (other.group1() + self.group1().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<CircleOnOrigin> for VersorEven {
    fn add_assign(&mut self, other: CircleOnOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() + self.group0().xyz()).with_w(self[e12345]),
            // e415, e425, e435, e321
            (other.group1() + self.group1().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<CircleOrthogonalOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        7        0        0
    fn add(self, other: CircleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0().xyz() + self.group0().xyz()).with_w(self[e12345]),
            // e415, e425, e435, e321
            self.group1().xyz().with_w(other[e321] + self[e321]),
            // e235, e315, e125, e5
            (other.group1() + self.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<CircleOrthogonalOrigin> for VersorEven {
    fn add_assign(&mut self, other: CircleOrthogonalOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0().xyz() + self.group0().xyz()).with_w(self[e12345]),
            // e415, e425, e435, e321
            self.group1().xyz().with_w(other[e321] + self[e321]),
            // e235, e315, e125, e5
            (other.group1() + self.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<CircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       11        0        0
    fn add(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + other.group0().with_w(other[e12345]),
            // e415, e425, e435, e321
            other.group1() + self.group1(),
            // e235, e315, e125, e5
            (other.group2().xyz() + self.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<CircleRotor> for VersorEven {
    fn add_assign(&mut self, other: CircleRotor) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + other.group0().with_w(other[e12345]),
            // e415, e425, e435, e321
            other.group1() + self.group1(),
            // e235, e315, e125, e5
            (other.group2().xyz() + self.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<CircleRotorAligningOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn add(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + other.group0().with_w(other[e12345]),
            // e415, e425, e435, e321
            (other.group1() + self.group1().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            (other.group2().xyz() + self.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<CircleRotorAligningOrigin> for VersorEven {
    fn add_assign(&mut self, other: CircleRotorAligningOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + other.group0().with_w(other[e12345]),
            // e415, e425, e435, e321
            (other.group1() + self.group1().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            (other.group2().xyz() + self.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<CircleRotorAligningOriginAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        7        0        0
    fn add(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345] + self[e12345]),
            // e415, e425, e435, e321
            (other.group0() + self.group1().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            (other.group1().xyz() + self.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<CircleRotorAligningOriginAtInfinity> for VersorEven {
    fn add_assign(&mut self, other: CircleRotorAligningOriginAtInfinity) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345] + self[e12345]),
            // e415, e425, e435, e321
            (other.group0() + self.group1().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            (other.group1().xyz() + self.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<CircleRotorAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        8        0        0
    fn add(self, other: CircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345] + self[e12345]),
            // e415, e425, e435, e321
            other.group0() + self.group1(),
            // e235, e315, e125, e5
            (other.group1().xyz() + self.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<CircleRotorAtInfinity> for VersorEven {
    fn add_assign(&mut self, other: CircleRotorAtInfinity) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345] + self[e12345]),
            // e415, e425, e435, e321
            other.group0() + self.group1(),
            // e235, e315, e125, e5
            (other.group1().xyz() + self.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<CircleRotorOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: CircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() + self.group0(),
            // e415, e425, e435, e321
            (other.group1() + self.group1().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<CircleRotorOnOrigin> for VersorEven {
    fn add_assign(&mut self, other: CircleRotorOnOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() + self.group0(),
            // e415, e425, e435, e321
            (other.group1() + self.group1().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<Dipole> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(other[e45]),
            // e15, e25, e35
            other.group2(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleAligningOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleAtInfinity> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleAtOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(0.0),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleInversion> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(other[e45]),
            // e15, e25, e35
            other.group2().xyz(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionAligningOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            other.group1().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionAtInfinity> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionAtOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().xyz().with_w(0.0),
            // e15, e25, e35
            other.group1().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionOnOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            other.group1(),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleInversionOrthogonalOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().xyz().with_w(0.0),
            // e15, e25, e35
            other.group2().xyz(),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleOnOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleOrthogonalOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(0.0),
            // e15, e25, e35
            other.group2(),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DualNum> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345] + self[e12345]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3().xyz().with_w(other[e4] + self[e4]),
        );
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
            self.group2(),
            // e1, e2, e3, e4
            self.group3().xyz().with_w(other[e4] + self[e4]),
        );
    }
}
impl std::ops::Add<FlatOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<FlatPoint> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            other.group0().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<FlatPointAtInfinity> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: FlatPointAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group0(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Flector> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            other.group0().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<FlectorAtInfinity> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group0().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<FlectorOnOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Horizon> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<Infinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2() + Simd32x3::from(0.0).with_w(other[e5]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<Infinity> for VersorEven {
    fn add_assign(&mut self, other: Infinity) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2() + Simd32x3::from(0.0).with_w(other[e5]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<Line> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            (other.group0() + self.group1().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            (other.group1() + self.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<Line> for VersorEven {
    fn add_assign(&mut self, other: Line) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            (other.group0() + self.group1().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            (other.group1() + self.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<LineAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: LineAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            (other.group0() + self.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<LineAtInfinity> for VersorEven {
    fn add_assign(&mut self, other: LineAtInfinity) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            (other.group0() + self.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<LineOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: LineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            (other.group0() + self.group1().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<LineOnOrigin> for VersorEven {
    fn add_assign(&mut self, other: LineOnOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            (other.group0() + self.group1().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<Motor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        8        0        0
    fn add(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
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
impl std::ops::Add<MotorAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: MotorAtInfinity) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            other.group0() + self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<MotorAtInfinity> for VersorEven {
    fn add_assign(&mut self, other: MotorAtInfinity) {
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            other.group0() + self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<MotorOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: MotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345] + self[e12345]),
            // e415, e425, e435, e321
            (other.group0().xyz() + self.group1().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<MotorOnOrigin> for VersorEven {
    fn add_assign(&mut self, other: MotorOnOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345] + self[e12345]),
            // e415, e425, e435, e321
            (other.group0().xyz() + self.group1().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<MultiVector> for VersorEven {
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
            Simd32x2::from([0.0, self[e12345]]) + other.group0(),
            // e1, e2, e3, e4
            other.group1() + self.group3(),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            other.group3(),
            // e15, e25, e35
            other.group4(),
            // e23, e31, e12
            other.group5(),
            // e415, e425, e435, e321
            other.group6() + self.group1(),
            // e423, e431, e412
            other.group7() + self.group0().xyz(),
            // e235, e315, e125
            other.group8() + self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            other.group9(),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<MysteryCircle> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: MysteryCircle) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            other.group0() + self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<MysteryCircle> for VersorEven {
    fn add_assign(&mut self, other: MysteryCircle) {
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            other.group0() + self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<MysteryCircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + Simd32x3::from(0.0).with_w(other[e12345]),
            // e415, e425, e435, e321
            other.group0() + self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<MysteryCircleRotor> for VersorEven {
    fn add_assign(&mut self, other: MysteryCircleRotor) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + Simd32x3::from(0.0).with_w(other[e12345]),
            // e415, e425, e435, e321
            other.group0() + self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<MysteryDipole> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MysteryDipoleInversion> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MysteryVersorEven> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        8        0        0
    fn add(self, other: MysteryVersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345] + self[e12345]),
            // e415, e425, e435, e321
            other.group1() + self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            (self.group3().xyz() + other.group0().yzw()).with_w(self[e4]),
        );
    }
}
impl std::ops::AddAssign<MysteryVersorEven> for VersorEven {
    fn add_assign(&mut self, other: MysteryVersorEven) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345] + self[e12345]),
            // e415, e425, e435, e321
            other.group1() + self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            (self.group3().xyz() + other.group0().yzw()).with_w(self[e4]),
        );
    }
}
impl std::ops::Add<MysteryVersorOdd> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullCircleAtOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: NullCircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() + self.group0().xyz()).with_w(self[e12345]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<NullCircleAtOrigin> for VersorEven {
    fn add_assign(&mut self, other: NullCircleAtOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() + self.group0().xyz()).with_w(self[e12345]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<NullDipoleAtOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: NullDipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullDipoleInversionAtOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().xyz().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullSphereAtOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullVersorEvenAtOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0().xyz() + self.group0().xyz()).with_w(self[e12345]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3().xyz().with_w(other[e4] + self[e4]),
        );
    }
}
impl std::ops::AddAssign<NullVersorEvenAtOrigin> for VersorEven {
    fn add_assign(&mut self, other: NullVersorEvenAtOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0().xyz() + self.group0().xyz()).with_w(self[e12345]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3().xyz().with_w(other[e4] + self[e4]),
        );
    }
}
impl std::ops::Add<Origin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3() + Simd32x3::from(0.0).with_w(other[e4]),
        );
    }
}
impl std::ops::AddAssign<Origin> for VersorEven {
    fn add_assign(&mut self, other: Origin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3() + Simd32x3::from(0.0).with_w(other[e4]),
        );
    }
}
impl std::ops::Add<Plane> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<PlaneOnOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<RoundPoint> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2() + Simd32x3::from(0.0).with_w(other[e5]),
            // e1, e2, e3, e4
            other.group0() + self.group3(),
        );
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
            self.group2() + Simd32x3::from(0.0).with_w(other[e5]),
            // e1, e2, e3, e4
            other.group0() + self.group3(),
        );
    }
}
impl std::ops::Add<RoundPointAtOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2().xyz().with_w(other[e5] + self[e5]),
            // e1, e2, e3, e4
            self.group3().xyz().with_w(other[e4] + self[e4]),
        );
    }
}
impl std::ops::AddAssign<RoundPointAtOrigin> for VersorEven {
    fn add_assign(&mut self, other: RoundPointAtOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2().xyz().with_w(other[e5] + self[e5]),
            // e1, e2, e3, e4
            self.group3().xyz().with_w(other[e4] + self[e4]),
        );
    }
}
impl std::ops::Add<Scalar> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Sphere> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<SphereAtOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<SphereOnOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            other.group0().wxyz(),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<VersorEven> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        4        0        0
    // no simd       16        0        0
    fn add(self, other: VersorEven) -> Self::Output {
        return VersorEven::from_groups(
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
impl std::ops::Add<VersorEvenAligningOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        4        0        0
    // no simd       16        0        0
    fn add(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + other.group0(),
            // e415, e425, e435, e321
            self.group1() + other.group1().xyz().with_w(0.0),
            // e235, e315, e125, e5
            self.group2() + other.group2(),
            // e1, e2, e3, e4
            self.group3() + Simd32x3::from(0.0).with_w(other[e4]),
        );
    }
}
impl std::ops::AddAssign<VersorEvenAligningOrigin> for VersorEven {
    fn add_assign(&mut self, other: VersorEvenAligningOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + other.group0(),
            // e415, e425, e435, e321
            self.group1() + other.group1().xyz().with_w(0.0),
            // e235, e315, e125, e5
            self.group2() + other.group2(),
            // e1, e2, e3, e4
            self.group3() + Simd32x3::from(0.0).with_w(other[e4]),
        );
    }
}
impl std::ops::Add<VersorEvenAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        4        0        0
    // no simd       16        0        0
    fn add(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + Simd32x3::from(0.0).with_w(other[e12345]),
            // e415, e425, e435, e321
            self.group1() + other.group1(),
            // e235, e315, e125, e5
            self.group2() + other.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]) + self.group3(),
        );
    }
}
impl std::ops::AddAssign<VersorEvenAtInfinity> for VersorEven {
    fn add_assign(&mut self, other: VersorEvenAtInfinity) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + Simd32x3::from(0.0).with_w(other[e12345]),
            // e415, e425, e435, e321
            self.group1() + other.group1(),
            // e235, e315, e125, e5
            self.group2() + other.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]) + self.group3(),
        );
    }
}
impl std::ops::Add<VersorEvenAtOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        3        0        0
    // no simd       12        0        0
    fn add(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + other.group0().xyz().with_w(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2() + other.group1(),
            // e1, e2, e3, e4
            self.group3() + Simd32x3::from(0.0).with_w(other[e4]),
        );
    }
}
impl std::ops::AddAssign<VersorEvenAtOrigin> for VersorEven {
    fn add_assign(&mut self, other: VersorEvenAtOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + other.group0().xyz().with_w(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2() + other.group1(),
            // e1, e2, e3, e4
            self.group3() + Simd32x3::from(0.0).with_w(other[e4]),
        );
    }
}
impl std::ops::Add<VersorEvenOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        3        0        0
    // no simd       12        0        0
    fn add(self, other: VersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + other.group0(),
            // e415, e425, e435, e321
            self.group1() + other.group1().xyz().with_w(0.0),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3() + Simd32x3::from(0.0).with_w(other[e4]),
        );
    }
}
impl std::ops::AddAssign<VersorEvenOnOrigin> for VersorEven {
    fn add_assign(&mut self, other: VersorEvenOnOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + other.group0(),
            // e415, e425, e435, e321
            self.group1() + other.group1().xyz().with_w(0.0),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3() + Simd32x3::from(0.0).with_w(other[e4]),
        );
    }
}
impl std::ops::Add<VersorEvenOrthogonalOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        4        0        0
    // no simd       16        0        0
    fn add(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + other.group0().xyz().with_w(0.0),
            // e415, e425, e435, e321
            self.group1() + Simd32x3::from(0.0).with_w(other[e321]),
            // e235, e315, e125, e5
            self.group2() + other.group1(),
            // e1, e2, e3, e4
            self.group3() + other.group2(),
        );
    }
}
impl std::ops::AddAssign<VersorEvenOrthogonalOrigin> for VersorEven {
    fn add_assign(&mut self, other: VersorEvenOrthogonalOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + other.group0().xyz().with_w(0.0),
            // e415, e425, e435, e321
            self.group1() + Simd32x3::from(0.0).with_w(other[e321]),
            // e235, e315, e125, e5
            self.group2() + other.group1(),
            // e1, e2, e3, e4
            self.group3() + other.group2(),
        );
    }
}
impl std::ops::Add<VersorOdd> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], other[e45]]),
            // e15, e25, e35
            other.group2().xyz(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<VersorOddAtInfinity> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            other.group0().yzw(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<VersorOddOrthogonalOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().xyz().with_w(0.0),
            // e15, e25, e35
            other.group2().xyz(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            other[e3215],
        );
    }
}

impl From<AntiDipoleInversion> for VersorEven {
    fn from(from_anti_dipole_inversion: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_anti_dipole_inversion.group0().with_w(0.0),
            // e415, e425, e435, e321
            from_anti_dipole_inversion.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([
                from_anti_dipole_inversion[e235],
                from_anti_dipole_inversion[e315],
                from_anti_dipole_inversion[e125],
                from_anti_dipole_inversion[e5],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                from_anti_dipole_inversion[e1],
                from_anti_dipole_inversion[e2],
                from_anti_dipole_inversion[e3],
                from_anti_dipole_inversion[e4],
            ]),
        );
    }
}

impl From<AntiDipoleInversionAtInfinity> for VersorEven {
    fn from(from_anti_dipole_inversion_at_infinity: AntiDipoleInversionAtInfinity) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            from_anti_dipole_inversion_at_infinity.group0(),
            // e235, e315, e125, e5
            from_anti_dipole_inversion_at_infinity.group1().with_w(from_anti_dipole_inversion_at_infinity[e5]),
            // e1, e2, e3, e4
            from_anti_dipole_inversion_at_infinity.group2().xyz().with_w(0.0),
        );
    }
}

impl From<AntiDipoleInversionOnOrigin> for VersorEven {
    fn from(from_anti_dipole_inversion_on_origin: AntiDipoleInversionOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_anti_dipole_inversion_on_origin.group0().xyz().with_w(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_dipole_inversion_on_origin[e321]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            from_anti_dipole_inversion_on_origin.group1().yzwx(),
        );
    }
}

impl From<AntiDipoleInversionOrthogonalOrigin> for VersorEven {
    fn from(from_anti_dipole_inversion_orthogonal_origin: AntiDipoleInversionOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_anti_dipole_inversion_orthogonal_origin.group0().xyz().with_w(0.0),
            // e415, e425, e435, e321
            from_anti_dipole_inversion_orthogonal_origin.group1().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                from_anti_dipole_inversion_orthogonal_origin[e235],
                from_anti_dipole_inversion_orthogonal_origin[e315],
                from_anti_dipole_inversion_orthogonal_origin[e125],
                from_anti_dipole_inversion_orthogonal_origin[e5],
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_anti_dipole_inversion_orthogonal_origin[e4]),
        );
    }
}

impl From<AntiDipoleOnOrigin> for VersorEven {
    fn from(from_anti_dipole_on_origin: AntiDipoleOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_anti_dipole_on_origin.group0().xyz().with_w(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_dipole_on_origin[e321]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiFlatOrigin> for VersorEven {
    fn from(from_anti_flat_origin: AntiFlatOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flat_origin[e321]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiFlatPoint> for VersorEven {
    fn from(from_anti_flat_point: AntiFlatPoint) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flat_point[e321]),
            // e235, e315, e125, e5
            from_anti_flat_point.group0().xyz().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiFlector> for VersorEven {
    fn from(from_anti_flector: AntiFlector) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flector[e321]),
            // e235, e315, e125, e5
            Simd32x4::from([from_anti_flector[e235], from_anti_flector[e315], from_anti_flector[e125], from_anti_flector[e5]]),
            // e1, e2, e3, e4
            from_anti_flector.group1().xyz().with_w(0.0),
        );
    }
}

impl From<AntiFlectorOnOrigin> for VersorEven {
    fn from(from_anti_flector_on_origin: AntiFlectorOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flector_on_origin[e321]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([from_anti_flector_on_origin[e1], from_anti_flector_on_origin[e2], from_anti_flector_on_origin[e3], 0.0]),
        );
    }
}

impl From<AntiMysteryDipoleInversion> for VersorEven {
    fn from(from_anti_mystery_dipole_inversion: AntiMysteryDipoleInversion) -> Self {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            from_anti_mystery_dipole_inversion.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            from_anti_mystery_dipole_inversion.group1().with_w(0.0),
        );
    }
}

impl From<AntiPlane> for VersorEven {
    fn from(from_anti_plane: AntiPlane) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(from_anti_plane[e5]),
            // e1, e2, e3, e4
            from_anti_plane.group0().xyz().with_w(0.0),
        );
    }
}

impl From<AntiPlaneOnOrigin> for VersorEven {
    fn from(from_anti_plane_on_origin: AntiPlaneOnOrigin) -> Self {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            from_anti_plane_on_origin.group0().with_w(0.0),
        );
    }
}

impl From<AntiScalar> for VersorEven {
    fn from(from_anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(from_anti_scalar[e12345]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiSphereOnOrigin> for VersorEven {
    fn from(from_anti_sphere_on_origin: AntiSphereOnOrigin) -> Self {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            from_anti_sphere_on_origin.group0(),
        );
    }
}

impl From<Circle> for VersorEven {
    fn from(from_circle: Circle) -> Self {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_circle.group0().with_w(0.0),
            // e415, e425, e435, e321
            from_circle.group1(),
            // e235, e315, e125, e5
            from_circle.group2().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleAligningOrigin> for VersorEven {
    fn from(from_circle_aligning_origin: CircleAligningOrigin) -> Self {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_circle_aligning_origin.group0().with_w(0.0),
            // e415, e425, e435, e321
            from_circle_aligning_origin.group1().with_w(0.0),
            // e235, e315, e125, e5
            from_circle_aligning_origin.group2().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleAtInfinity> for VersorEven {
    fn from(from_circle_at_infinity: CircleAtInfinity) -> Self {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            from_circle_at_infinity.group0(),
            // e235, e315, e125, e5
            from_circle_at_infinity.group1().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleAtOrigin> for VersorEven {
    fn from(from_circle_at_origin: CircleAtOrigin) -> Self {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_circle_at_origin.group0().with_w(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            from_circle_at_origin.group1().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleOnOrigin> for VersorEven {
    fn from(from_circle_on_origin: CircleOnOrigin) -> Self {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_circle_on_origin.group0().with_w(0.0),
            // e415, e425, e435, e321
            from_circle_on_origin.group1().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleOrthogonalOrigin> for VersorEven {
    fn from(from_circle_orthogonal_origin: CircleOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_circle_orthogonal_origin.group0().xyz().with_w(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_circle_orthogonal_origin[e321]),
            // e235, e315, e125, e5
            from_circle_orthogonal_origin.group1().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleRotor> for VersorEven {
    fn from(from_circle_rotor: CircleRotor) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_circle_rotor.group0().with_w(from_circle_rotor[e12345]),
            // e415, e425, e435, e321
            from_circle_rotor.group1(),
            // e235, e315, e125, e5
            from_circle_rotor.group2().xyz().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleRotorAligningOrigin> for VersorEven {
    fn from(from_circle_rotor_aligning_origin: CircleRotorAligningOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_circle_rotor_aligning_origin.group0().with_w(from_circle_rotor_aligning_origin[e12345]),
            // e415, e425, e435, e321
            from_circle_rotor_aligning_origin.group1().with_w(0.0),
            // e235, e315, e125, e5
            from_circle_rotor_aligning_origin.group2().xyz().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleRotorAligningOriginAtInfinity> for VersorEven {
    fn from(from_circle_rotor_aligning_origin_at_infinity: CircleRotorAligningOriginAtInfinity) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(from_circle_rotor_aligning_origin_at_infinity[e12345]),
            // e415, e425, e435, e321
            from_circle_rotor_aligning_origin_at_infinity.group0().with_w(0.0),
            // e235, e315, e125, e5
            from_circle_rotor_aligning_origin_at_infinity.group1().xyz().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleRotorAtInfinity> for VersorEven {
    fn from(from_circle_rotor_at_infinity: CircleRotorAtInfinity) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(from_circle_rotor_at_infinity[e12345]),
            // e415, e425, e435, e321
            from_circle_rotor_at_infinity.group0(),
            // e235, e315, e125, e5
            from_circle_rotor_at_infinity.group1().xyz().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleRotorOnOrigin> for VersorEven {
    fn from(from_circle_rotor_on_origin: CircleRotorOnOrigin) -> Self {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_circle_rotor_on_origin.group0(),
            // e415, e425, e435, e321
            from_circle_rotor_on_origin.group1().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<DualNum> for VersorEven {
    fn from(from_dual_num: DualNum) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(from_dual_num[e12345]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_dual_num[e4]),
        );
    }
}

impl From<Infinity> for VersorEven {
    fn from(from_infinity: Infinity) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(from_infinity[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<Line> for VersorEven {
    fn from(from_line: Line) -> Self {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            from_line.group0().with_w(0.0),
            // e235, e315, e125, e5
            from_line.group1().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<LineAtInfinity> for VersorEven {
    fn from(from_line_at_infinity: LineAtInfinity) -> Self {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            from_line_at_infinity.group0().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<LineOnOrigin> for VersorEven {
    fn from(from_line_on_origin: LineOnOrigin) -> Self {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            from_line_on_origin.group0().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<Motor> for VersorEven {
    fn from(from_motor: Motor) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(from_motor[e12345]),
            // e415, e425, e435, e321
            from_motor.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            from_motor.group1(),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<MotorAtInfinity> for VersorEven {
    fn from(from_motor_at_infinity: MotorAtInfinity) -> Self {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            from_motor_at_infinity.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<MotorOnOrigin> for VersorEven {
    fn from(from_motor_on_origin: MotorOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(from_motor_on_origin[e12345]),
            // e415, e425, e435, e321
            from_motor_on_origin.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<MysteryCircle> for VersorEven {
    fn from(from_mystery_circle: MysteryCircle) -> Self {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            from_mystery_circle.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<MysteryCircleRotor> for VersorEven {
    fn from(from_mystery_circle_rotor: MysteryCircleRotor) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(from_mystery_circle_rotor[e12345]),
            // e415, e425, e435, e321
            from_mystery_circle_rotor.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<MysteryVersorEven> for VersorEven {
    fn from(from_mystery_versor_even: MysteryVersorEven) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(from_mystery_versor_even[e12345]),
            // e415, e425, e435, e321
            from_mystery_versor_even.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([from_mystery_versor_even[e1], from_mystery_versor_even[e2], from_mystery_versor_even[e3], 0.0]),
        );
    }
}

impl From<NullCircleAtOrigin> for VersorEven {
    fn from(from_null_circle_at_origin: NullCircleAtOrigin) -> Self {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_null_circle_at_origin.group0().with_w(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<NullVersorEvenAtOrigin> for VersorEven {
    fn from(from_null_versor_even_at_origin: NullVersorEvenAtOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_null_versor_even_at_origin.group0().xyz().with_w(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_null_versor_even_at_origin[e4]),
        );
    }
}

impl From<Origin> for VersorEven {
    fn from(from_origin: Origin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_origin[e4]),
        );
    }
}

impl From<RoundPoint> for VersorEven {
    fn from(from_round_point: RoundPoint) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(from_round_point[e5]),
            // e1, e2, e3, e4
            from_round_point.group0(),
        );
    }
}

impl From<RoundPointAtOrigin> for VersorEven {
    fn from(from_round_point_at_origin: RoundPointAtOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(from_round_point_at_origin[e5]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_round_point_at_origin[e4]),
        );
    }
}

impl From<VersorEvenAligningOrigin> for VersorEven {
    fn from(from_versor_even_aligning_origin: VersorEvenAligningOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_versor_even_aligning_origin.group0(),
            // e415, e425, e435, e321
            from_versor_even_aligning_origin.group1().xyz().with_w(0.0),
            // e235, e315, e125, e5
            from_versor_even_aligning_origin.group2(),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_versor_even_aligning_origin[e4]),
        );
    }
}

impl From<VersorEvenAtInfinity> for VersorEven {
    fn from(from_versor_even_at_infinity: VersorEvenAtInfinity) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(from_versor_even_at_infinity[e12345]),
            // e415, e425, e435, e321
            from_versor_even_at_infinity.group1(),
            // e235, e315, e125, e5
            from_versor_even_at_infinity.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([from_versor_even_at_infinity[e1], from_versor_even_at_infinity[e2], from_versor_even_at_infinity[e3], 0.0]),
        );
    }
}

impl From<VersorEvenAtOrigin> for VersorEven {
    fn from(from_versor_even_at_origin: VersorEvenAtOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_versor_even_at_origin.group0().xyz().with_w(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            from_versor_even_at_origin.group1(),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_versor_even_at_origin[e4]),
        );
    }
}

impl From<VersorEvenOnOrigin> for VersorEven {
    fn from(from_versor_even_on_origin: VersorEvenOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_versor_even_on_origin.group0(),
            // e415, e425, e435, e321
            from_versor_even_on_origin.group1().xyz().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_versor_even_on_origin[e4]),
        );
    }
}

impl From<VersorEvenOrthogonalOrigin> for VersorEven {
    fn from(from_versor_even_orthogonal_origin: VersorEvenOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_versor_even_orthogonal_origin.group0().xyz().with_w(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_versor_even_orthogonal_origin[e321]),
            // e235, e315, e125, e5
            from_versor_even_orthogonal_origin.group1(),
            // e1, e2, e3, e4
            from_versor_even_orthogonal_origin.group2(),
        );
    }
}
impl std::ops::Mul<AntiCircleOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       57        0
    //    simd3        0        8        0
    //    simd4       12        4        0
    // Totals...
    // yes simd       44       69        0
    //  no simd       80       97        0
    fn mul(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleOnOrigin> for VersorEven {
    fn mul_assign(&mut self, other: AntiCircleOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       76        0
    //    simd3        0       12        0
    //    simd4       28       16        0
    // Totals...
    // yes simd       76      104        0
    //  no simd      160      176        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotor> for VersorEven {
    fn mul_assign(&mut self, other: AntiCircleRotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       77        0
    //    simd3        0       13        0
    //    simd4       24       11        0
    // Totals...
    // yes simd       72      101        0
    //  no simd      144      160        0
    fn mul(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorAligningOrigin> for VersorEven {
    fn mul_assign(&mut self, other: AntiCircleRotorAligningOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOriginAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       55        0
    //    simd3        0        6        0
    //    simd4       16       10        0
    // Totals...
    // yes simd       48       71        0
    //  no simd       96      113        0
    fn mul(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorAligningOriginAtInfinity> for VersorEven {
    fn mul_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       51        0
    //    simd3        0        3        0
    //    simd4       20       17        0
    // Totals...
    // yes simd       53       71        0
    //  no simd      113      128        0
    fn mul(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorAtInfinity> for VersorEven {
    fn mul_assign(&mut self, other: AntiCircleRotorAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       56        0
    //    simd3        0        7        0
    //    simd4       16        9        0
    // Totals...
    // yes simd       48       72        0
    //  no simd       96      113        0
    fn mul(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorOnOrigin> for VersorEven {
    fn mul_assign(&mut self, other: AntiCircleRotorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       71        0
    //    simd3        0       17        0
    //    simd4       48       31        0
    // Totals...
    // yes simd       80      119        0
    //  no simd      224      246        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionAtInfinity> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       47        0
    //    simd3        0        3        0
    //    simd4       34       31        0
    // Totals...
    // yes simd       62       81        0
    //  no simd      164      180        0
    fn mul(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOnOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       50        0
    //    simd3        0        6        0
    //    simd4       21       15        0
    // Totals...
    // yes simd       53       71        0
    //  no simd      116      128        0
    fn mul(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOrthogonalOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       67        0
    //    simd3        0       13        0
    //    simd4       32       19        0
    // Totals...
    // yes simd       64       99        0
    //  no simd      160      182        0
    fn mul(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleOnOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       30        0
    //    simd3        3        6        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       28       41        0
    //  no simd       49       68        0
    fn mul(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1       16        0
    //    simd4        4        5        0
    // Totals...
    // yes simd        5       21        0
    //  no simd       17       36        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiDualNum> for VersorEven {
    fn mul_assign(&mut self, other: AntiDualNum) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        8        0
    // no simd        0       32        0
    fn mul(self, other: AntiFlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        8        0
    //    simd3        6        8        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       18       24        0
    //  no simd       51       64        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       34        0
    //    simd3        0        2        0
    //    simd4       23       22        0
    // Totals...
    // yes simd       50       58        0
    //  no simd      119      128        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlectorOnOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4       12       16        0
    // no simd       48       64        0
    fn mul(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       55        0
    //    simd3        0        6        0
    //    simd4       12        6        0
    // Totals...
    // yes simd       44       67        0
    //  no simd       80       97        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiLine> for VersorEven {
    fn mul_assign(&mut self, other: AntiLine) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLineOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       35        0
    //    simd3        0        3        0
    //    simd4        4        1        0
    // Totals...
    // yes simd       20       39        0
    //  no simd       32       48        0
    fn mul(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiLineOnOrigin> for VersorEven {
    fn mul_assign(&mut self, other: AntiLineOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       52        0
    //    simd3        0        3        0
    //    simd4       20       17        0
    // Totals...
    // yes simd       52       72        0
    //  no simd      112      129        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMotor> for VersorEven {
    fn mul_assign(&mut self, other: AntiMotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotorOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       32        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       24       40        0
    //  no simd       48       64        0
    fn mul(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMotorOnOrigin> for VersorEven {
    fn mul_assign(&mut self, other: AntiMotorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryCircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       34        0
    //    simd3        0        1        0
    //    simd4       12       11        0
    // Totals...
    // yes simd       28       46        0
    //  no simd       64       81        0
    fn mul(self, other: AntiMysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMysteryCircleRotor> for VersorEven {
    fn mul_assign(&mut self, other: AntiMysteryCircleRotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryDipoleInversion> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       36        0
    //    simd3        0        8        0
    //    simd4       22       14        0
    // Totals...
    // yes simd       30       58        0
    //  no simd       96      116        0
    fn mul(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd4       13       14        0
    // Totals...
    // yes simd       16       22        0
    //  no simd       55       64        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlaneOnOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       26        0
    //    simd3        0        2        0
    //    simd4        8        6        0
    // Totals...
    // yes simd        8       34        0
    //  no simd       32       56        0
    fn mul(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        8        0
    // no simd        0       32        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiSphereOnOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       25        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       18       35        0
    //  no simd       48       65        0
    fn mul(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorEvenOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       53        0
    //    simd3        0        5        0
    //    simd4       20       15        0
    // Totals...
    // yes simd       52       73        0
    //  no simd      112      128        0
    fn mul(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiVersorEvenOnOrigin> for VersorEven {
    fn mul_assign(&mut self, other: AntiVersorEvenOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       63        0
    //    simd3        0       12        0
    //    simd4       27       16        0
    // Totals...
    // yes simd       63       91        0
    //  no simd      144      163        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAligningOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       61        0
    //    simd3        0       10        0
    //    simd4       23       14        0
    // Totals...
    // yes simd       59       85        0
    //  no simd      128      147        0
    fn mul(self, other: CircleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtInfinity> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       37        0
    //    simd3        0        1        0
    //    simd4       20       19        0
    // Totals...
    // yes simd       40       57        0
    //  no simd      100      116        0
    fn mul(self, other: CircleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       44        0
    //    simd3        0        9        0
    //    simd4       15        7        0
    // Totals...
    // yes simd       35       60        0
    //  no simd       80       99        0
    fn mul(self, other: CircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOnOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       58        0
    //    simd3        0        7        0
    //    simd4       12        5        0
    // Totals...
    // yes simd       45       70        0
    //  no simd       81       99        0
    fn mul(self, other: CircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOrthogonalOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       49        0
    //    simd3        0        8        0
    //    simd4       18       10        0
    // Totals...
    // yes simd       42       67        0
    //  no simd       96      113        0
    fn mul(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       68        0
    //    simd3        0       11        0
    //    simd4       30       19        0
    // Totals...
    // yes simd       70       98        0
    //  no simd      160      177        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       62        0
    //    simd3        0       11        0
    //    simd4       27       17        0
    // Totals...
    // yes simd       63       90        0
    //  no simd      144      163        0
    fn mul(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOriginAtInfinity> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       40        0
    //    simd3        0        3        0
    //    simd4       19       16        0
    // Totals...
    // yes simd       43       59        0
    //  no simd      100      113        0
    fn mul(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAtInfinity> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd3        0        1        0
    //    simd4       24       24        0
    // Totals...
    // yes simd       44       57        0
    //  no simd      116      131        0
    fn mul(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorOnOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       37       59        0
    //    simd3        0        7        0
    //    simd4       15        8        0
    // Totals...
    // yes simd       52       74        0
    //  no simd       97      112        0
    fn mul(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       73        0
    //    simd3        0        9        0
    //    simd4       24       15        0
    // Totals...
    // yes simd       72       97        0
    //  no simd      144      160        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Dipole> for VersorEven {
    fn mul_assign(&mut self, other: Dipole) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAligningOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       38        0
    //    simd3        0        5        0
    //    simd4       19       15        0
    // Totals...
    // yes simd       39       58        0
    //  no simd       96      113        0
    fn mul(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleAligningOrigin> for VersorEven {
    fn mul_assign(&mut self, other: DipoleAligningOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       50        0
    //    simd3        0        2        0
    //    simd4       16       14        0
    // Totals...
    // yes simd       49       66        0
    //  no simd       97      112        0
    fn mul(self, other: DipoleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleAtInfinity> for VersorEven {
    fn mul_assign(&mut self, other: DipoleAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       39        0
    //    simd3        0        6        0
    //    simd4       15       10        0
    // Totals...
    // yes simd       35       55        0
    //  no simd       80       97        0
    fn mul(self, other: DipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleAtOrigin> for VersorEven {
    fn mul_assign(&mut self, other: DipoleAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       65        0
    //    simd3        0       17        0
    //    simd4       47       31        0
    // Totals...
    // yes simd       83      113        0
    //  no simd      224      240        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversion> for VersorEven {
    fn mul_assign(&mut self, other: DipoleInversion) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAligningOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       38        0
    //    simd3        0        6        0
    //    simd4       39       34        0
    // Totals...
    // yes simd       59       78        0
    //  no simd      176      192        0
    fn mul(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionAligningOrigin> for VersorEven {
    fn mul_assign(&mut self, other: DipoleInversionAligningOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       36        0
    //    simd3        0        3        0
    //    simd4       35       33        0
    // Totals...
    // yes simd       55       72        0
    //  no simd      160      177        0
    fn mul(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionAtInfinity> for VersorEven {
    fn mul_assign(&mut self, other: DipoleInversionAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       37        0
    //    simd3        0        5        0
    //    simd4       23       19        0
    // Totals...
    // yes simd       43       61        0
    //  no simd      112      128        0
    fn mul(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionAtOrigin> for VersorEven {
    fn mul_assign(&mut self, other: DipoleInversionAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       40        0
    //    simd3        0        3        0
    //    simd4       23       20        0
    // Totals...
    // yes simd       46       63        0
    //  no simd      115      129        0
    fn mul(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionOnOrigin> for VersorEven {
    fn mul_assign(&mut self, other: DipoleInversionOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOrthogonalOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       78        0
    //    simd3        0       14        0
    //    simd4       28       14        0
    // Totals...
    // yes simd       76      106        0
    //  no simd      160      176        0
    fn mul(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionOrthogonalOrigin> for VersorEven {
    fn mul_assign(&mut self, other: DipoleInversionOrthogonalOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       27        0
    //    simd4        9       11        0
    // Totals...
    // yes simd       24       38        0
    //  no simd       51       71        0
    fn mul(self, other: DipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleOnOrigin> for VersorEven {
    fn mul_assign(&mut self, other: DipoleOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOrthogonalOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       76        0
    //    simd3        0       12        0
    //    simd4       20        8        0
    // Totals...
    // yes simd       68       96        0
    //  no simd      128      144        0
    fn mul(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleOrthogonalOrigin> for VersorEven {
    fn mul_assign(&mut self, other: DipoleOrthogonalOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       18        0
    //    simd3        1        2        0
    //    simd4        3        5        0
    // Totals...
    // yes simd        6       25        0
    //  no simd       17       44        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        8        0
    // no simd        0       32        0
    fn mul(self, other: FlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlatOrigin> for VersorEven {
    fn mul_assign(&mut self, other: FlatOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        3        5        0
    //    simd4        8        9        0
    // Totals...
    // yes simd       18       27        0
    //  no simd       48       64        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlatPoint> for VersorEven {
    fn mul_assign(&mut self, other: FlatPoint) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPointAtInfinity> for VersorEven {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       15       24        0
    //  no simd       36       48        0
    fn mul(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       26        0
    //    simd3        0        2        0
    //    simd4       25       24        0
    // Totals...
    // yes simd       41       52        0
    //  no simd      116      128        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Flector> for VersorEven {
    fn mul_assign(&mut self, other: Flector) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorAtInfinity> for VersorEven {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4       11       12        0
    // Totals...
    // yes simd       19       28        0
    //  no simd       52       64        0
    fn mul(self, other: FlectorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4       10       12        0
    // Totals...
    // yes simd       18       28        0
    //  no simd       48       64        0
    fn mul(self, other: FlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlectorOnOrigin> for VersorEven {
    fn mul_assign(&mut self, other: FlectorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for VersorEven {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        4        0
    // no simd        4       16        0
    fn mul(self, other: Horizon) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Infinity> for VersorEven {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        4       13        0
    //  no simd        4       24        0
    fn mul(self, other: Infinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       43        0
    //    simd3        0        3        0
    //    simd4       14       11        0
    // Totals...
    // yes simd       42       57        0
    //  no simd       84       96        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineAtInfinity> for VersorEven {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       18        0
    //    simd3        0        1        0
    //    simd4        8        7        0
    // Totals...
    // yes simd       16       26        0
    //  no simd       40       49        0
    fn mul(self, other: LineAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineOnOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       30        0
    //    simd3        0        2        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       14       36        0
    //  no simd       32       52        0
    fn mul(self, other: LineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       39        0
    //    simd4       22       23        0
    // Totals...
    // yes simd       50       62        0
    //  no simd      116      131        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorAtInfinity> for VersorEven {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       16        0
    //    simd4       11       12        0
    // Totals...
    // yes simd       23       28        0
    //  no simd       56       64        0
    fn mul(self, other: MotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorOnOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4       10       12        0
    // Totals...
    // yes simd       18       28        0
    //  no simd       48       64        0
    fn mul(self, other: MotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       68       94        0
    //    simd2        8        8        0
    //    simd3       60       74        0
    //    simd4       54       45        0
    // Totals...
    // yes simd      190      221        0
    //  no simd      480      512        0
    fn mul(self, other: MultiVector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircle> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       19        0
    //    simd4       10       12        0
    // Totals...
    // yes simd       18       31        0
    //  no simd       48       67        0
    fn mul(self, other: MysteryCircle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircleRotor> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       32        0
    //    simd3        0        1        0
    //    simd4       14       13        0
    // Totals...
    // yes simd       22       46        0
    //  no simd       64       87        0
    fn mul(self, other: MysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipole> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       33        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       24       41        0
    //  no simd       48       65        0
    fn mul(self, other: MysteryDipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryDipole> for VersorEven {
    fn mul_assign(&mut self, other: MysteryDipole) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipoleInversion> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       20        0
    //    simd3        0        4        0
    //    simd4       22       20        0
    // Totals...
    // yes simd       30       44        0
    //  no simd       96      112        0
    fn mul(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryDipoleInversion> for VersorEven {
    fn mul_assign(&mut self, other: MysteryDipoleInversion) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorEven> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       20        0
    //    simd3        0        4        0
    //    simd4       26       24        0
    // Totals...
    // yes simd       34       48        0
    //  no simd      112      128        0
    fn mul(self, other: MysteryVersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorOdd> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       19        0
    //    simd3        0        3        0
    //    simd4       26       25        0
    // Totals...
    // yes simd       34       47        0
    //  no simd      112      128        0
    fn mul(self, other: MysteryVersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryVersorOdd> for VersorEven {
    fn mul_assign(&mut self, other: MysteryVersorOdd) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<NullCircleAtOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       29        0
    //    simd3        0        5        0
    //    simd4        5        1        0
    // Totals...
    // yes simd       23       35        0
    //  no simd       38       48        0
    fn mul(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleAtOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       32        0
    //    simd3        0        4        0
    //    simd4        6        2        0
    // Totals...
    // yes simd       18       38        0
    //  no simd       36       52        0
    fn mul(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<NullDipoleAtOrigin> for VersorEven {
    fn mul_assign(&mut self, other: NullDipoleAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleInversionAtOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       21        0
    //    simd4       10       11        0
    // Totals...
    // yes simd       25       32        0
    //  no simd       55       65        0
    fn mul(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<NullDipoleInversionAtOrigin> for VersorEven {
    fn mul_assign(&mut self, other: NullDipoleInversionAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<NullSphereAtOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        1        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        9        0
    //  no simd        4       23        0
    fn mul(self, other: NullSphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<NullSphereAtOrigin> for VersorEven {
    fn mul_assign(&mut self, other: NullSphereAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<NullVersorEvenAtOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       32        0
    //    simd3        0        4        0
    //    simd4        8        5        0
    // Totals...
    // yes simd       31       41        0
    //  no simd       55       64        0
    fn mul(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Origin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        4        0
    //    simd3        1        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2       10        0
    //  no simd        4       24        0
    fn mul(self, other: Origin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       32        0
    //    simd4        9        9        0
    // Totals...
    // yes simd       21       41        0
    //  no simd       48       68        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Plane> for VersorEven {
    fn mul_assign(&mut self, other: Plane) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<PlaneOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       29        0
    //    simd3        0        1        0
    //    simd4        6        5        0
    // Totals...
    // yes simd       14       35        0
    //  no simd       32       52        0
    fn mul(self, other: PlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<PlaneOnOrigin> for VersorEven {
    fn mul_assign(&mut self, other: PlaneOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       28        0
    //    simd3        0        6        0
    //    simd4       15       10        0
    // Totals...
    // yes simd       19       44        0
    //  no simd       64       86        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPointAtOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       12        0
    //    simd4        4        6        0
    // Totals...
    // yes simd        4       18        0
    //  no simd       16       36        0
    fn mul(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn mul(self, other: Scalar) -> Self::Output {
        return self.geometric_product(other);
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
    //           add/sub      mul      div
    //      f32        4       28        0
    //    simd3        0        6        0
    //    simd4       15       10        0
    // Totals...
    // yes simd       19       44        0
    //  no simd       64       86        0
    fn mul(self, other: Sphere) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Sphere> for VersorEven {
    fn mul_assign(&mut self, other: Sphere) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereAtOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       12        0
    //    simd4        4        6        0
    // Totals...
    // yes simd        4       18        0
    //  no simd       16       36        0
    fn mul(self, other: SphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<SphereAtOrigin> for VersorEven {
    fn mul_assign(&mut self, other: SphereAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       22        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       16       34        0
    //  no simd       52       70        0
    fn mul(self, other: SphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<SphereOnOrigin> for VersorEven {
    fn mul_assign(&mut self, other: SphereOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       67        0
    //    simd3        0       19        0
    //    simd4       51       33        0
    // Totals...
    // yes simd       87      119        0
    //  no simd      240      256        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAligningOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       61        0
    //    simd3        0       13        0
    //    simd4       35       23        0
    // Totals...
    // yes simd       71       97        0
    //  no simd      176      192        0
    fn mul(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtInfinity> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       47        0
    //    simd3        0        8        0
    //    simd4       38       31        0
    // Totals...
    // yes simd       66       86        0
    //  no simd      180      195        0
    fn mul(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       42        0
    //    simd3        0       10        0
    //    simd4       23       14        0
    // Totals...
    // yes simd       43       66        0
    //  no simd      112      128        0
    fn mul(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOnOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       54        0
    //    simd3        0        2        0
    //    simd4       19       17        0
    // Totals...
    // yes simd       59       73        0
    //  no simd      116      128        0
    fn mul(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOrthogonalOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       46        0
    //    simd3        0       14        0
    //    simd4       39       26        0
    // Totals...
    // yes simd       59       86        0
    //  no simd      176      192        0
    fn mul(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       59        0
    //    simd3        0       11        0
    //    simd4       51       41        0
    // Totals...
    // yes simd       87      111        0
    //  no simd      240      256        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorOdd> for VersorEven {
    fn mul_assign(&mut self, other: VersorOdd) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       37        0
    //    simd3        0        4        0
    //    simd4       39       36        0
    // Totals...
    // yes simd       59       77        0
    //  no simd      176      193        0
    fn mul(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorOddAtInfinity> for VersorEven {
    fn mul_assign(&mut self, other: VersorOddAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddOrthogonalOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       73        0
    //    simd3        0        9        0
    //    simd4       32       23        0
    // Totals...
    // yes simd       80      105        0
    //  no simd      176      192        0
    fn mul(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorOddOrthogonalOrigin> for VersorEven {
    fn mul_assign(&mut self, other: VersorOddOrthogonalOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Neg for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn neg(self) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from(-1.0),
            // e1, e2, e3, e4
            self.group3() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Not for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiCircleOnOrigin> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: AntiCircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotor> for VersorEven {
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
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(other[e45]) * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOrigin> for VersorEven {
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
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOriginAtInfinity> for VersorEven {
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
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAtInfinity> for VersorEven {
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
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorOnOrigin> for VersorEven {
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
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiDipoleInversion> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        4        0        0
    // Totals...
    // yes simd        4        3        0
    //  no simd       16        3        0
    fn sub(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435, e321
            self.group1() - other.group1(),
            // e235, e315, e125, e5
            self.group2() - other.group2().xyz().with_w(other[e5]),
            // e1, e2, e3, e4
            self.group3() - other.group3().xyz().with_w(other[e4]),
        );
    }
}
impl std::ops::SubAssign<AntiDipoleInversion> for VersorEven {
    fn sub_assign(&mut self, other: AntiDipoleInversion) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435, e321
            self.group1() - other.group1(),
            // e235, e315, e125, e5
            self.group2() - other.group2().xyz().with_w(other[e5]),
            // e1, e2, e3, e4
            self.group3() - other.group3().xyz().with_w(other[e4]),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        3        3        0
    //  no simd       12        3        0
    fn sub(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1() - other.group0(),
            // e235, e315, e125, e5
            self.group2() - other.group1().with_w(other[e5]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group3(),
        );
    }
}
impl std::ops::SubAssign<AntiDipoleInversionAtInfinity> for VersorEven {
    fn sub_assign(&mut self, other: AntiDipoleInversionAtInfinity) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1() - other.group0(),
            // e235, e315, e125, e5
            self.group2() - other.group1().with_w(other[e5]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group3(),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        3        4        0
    //  no simd       12        4        0
    fn sub(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435, e321
            self.group1() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3() - other.group1().yzwx(),
        );
    }
}
impl std::ops::SubAssign<AntiDipoleInversionOnOrigin> for VersorEven {
    fn sub_assign(&mut self, other: AntiDipoleInversionOnOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435, e321
            self.group1() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3() - other.group1().yzwx(),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOrthogonalOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        7        0
    //    simd4        4        0        0
    // Totals...
    // yes simd        4        7        0
    //  no simd       16        7        0
    fn sub(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group1(),
            // e235, e315, e125, e5
            self.group2() - other.group2().xyz().with_w(other[e5]),
            // e1, e2, e3, e4
            self.group3() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::SubAssign<AntiDipoleInversionOrthogonalOrigin> for VersorEven {
    fn sub_assign(&mut self, other: AntiDipoleInversionOrthogonalOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group1(),
            // e235, e315, e125, e5
            self.group2() - other.group2().xyz().with_w(other[e5]),
            // e1, e2, e3, e4
            self.group3() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::Sub<AntiDipoleOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        4        0
    fn sub(self, other: AntiDipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435, e321
            self.group1() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<AntiDipoleOnOrigin> for VersorEven {
    fn sub_assign(&mut self, other: AntiDipoleOnOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435, e321
            self.group1() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<AntiDualNum> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        2        0
    // no simd        0        4        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiFlatOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<AntiFlatOrigin> for VersorEven {
    fn sub_assign(&mut self, other: AntiFlatOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<AntiFlatPoint> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        4        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<AntiFlatPoint> for VersorEven {
    fn sub_assign(&mut self, other: AntiFlatPoint) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<AntiFlector> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        3        4        0
    //  no simd       12        4        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125, e5
            self.group2() - other.group0().xyz().with_w(other[e5]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group3(),
        );
    }
}
impl std::ops::SubAssign<AntiFlector> for VersorEven {
    fn sub_assign(&mut self, other: AntiFlector) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125, e5
            self.group2() - other.group0().xyz().with_w(other[e5]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group3(),
        );
    }
}
impl std::ops::Sub<AntiFlectorOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        4        0
    fn sub(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group3(),
        );
    }
}
impl std::ops::SubAssign<AntiFlectorOnOrigin> for VersorEven {
    fn sub_assign(&mut self, other: AntiFlectorOnOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group3(),
        );
    }
}
impl std::ops::Sub<AntiLine> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiLineOnOrigin> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: AntiLineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiMotor> for VersorEven {
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
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<AntiMotorOnOrigin> for VersorEven {
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
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiMysteryCircleRotor> for VersorEven {
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
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiMysteryDipoleInversion> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        8        3        0
    fn sub(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1() - other.group0(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group3(),
        );
    }
}
impl std::ops::SubAssign<AntiMysteryDipoleInversion> for VersorEven {
    fn sub_assign(&mut self, other: AntiMysteryDipoleInversion) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1() - other.group0(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group3(),
        );
    }
}
impl std::ops::Sub<AntiPlane> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        4        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2() + Simd32x3::from(0.0).with_w(other[e5] * -1.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group3(),
        );
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
            self.group2() + Simd32x3::from(0.0).with_w(other[e5] * -1.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group3(),
        );
    }
}
impl std::ops::Sub<AntiPlaneOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        4        3        0
    fn sub(self, other: AntiPlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group3(),
        );
    }
}
impl std::ops::SubAssign<AntiPlaneOnOrigin> for VersorEven {
    fn sub_assign(&mut self, other: AntiPlaneOnOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group3(),
        );
    }
}
impl std::ops::Sub<AntiScalar> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + Simd32x3::from(0.0).with_w(other[e12345] * -1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<AntiScalar> for VersorEven {
    fn sub_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + Simd32x3::from(0.0).with_w(other[e12345] * -1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<AntiSphereOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: AntiSphereOnOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3() - other.group0(),
        );
    }
}
impl std::ops::SubAssign<AntiSphereOnOrigin> for VersorEven {
    fn sub_assign(&mut self, other: AntiSphereOnOrigin) {
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3() - other.group0(),
        );
    }
}
impl std::ops::Sub<AntiVersorEvenOnOrigin> for VersorEven {
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
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Circle> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        3        6        0
    //  no simd       12        6        0
    fn sub(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435, e321
            self.group1() - other.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<Circle> for VersorEven {
    fn sub_assign(&mut self, other: Circle) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435, e321
            self.group1() - other.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<CircleAligningOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        9        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        3        9        0
    //  no simd       12        9        0
    fn sub(self, other: CircleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<CircleAligningOrigin> for VersorEven {
    fn sub_assign(&mut self, other: CircleAligningOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<CircleAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        8        3        0
    fn sub(self, other: CircleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1() - other.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<CircleAtInfinity> for VersorEven {
    fn sub_assign(&mut self, other: CircleAtInfinity) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1() - other.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<CircleAtOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        8        6        0
    fn sub(self, other: CircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<CircleAtOrigin> for VersorEven {
    fn sub_assign(&mut self, other: CircleAtOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<CircleOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        8        6        0
    fn sub(self, other: CircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<CircleOnOrigin> for VersorEven {
    fn sub_assign(&mut self, other: CircleOnOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<CircleOrthogonalOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        7        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        3        7        0
    //  no simd       12        7        0
    fn sub(self, other: CircleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435, e321
            self.group1() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<CircleOrthogonalOrigin> for VersorEven {
    fn sub_assign(&mut self, other: CircleOrthogonalOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435, e321
            self.group1() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<CircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        3        3        0
    //  no simd       12        3        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() - other.group0().with_w(other[e12345]),
            // e415, e425, e435, e321
            self.group1() - other.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<CircleRotor> for VersorEven {
    fn sub_assign(&mut self, other: CircleRotor) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() - other.group0().with_w(other[e12345]),
            // e415, e425, e435, e321
            self.group1() - other.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        3        6        0
    //  no simd       12        6        0
    fn sub(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() - other.group0().with_w(other[e12345]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<CircleRotorAligningOrigin> for VersorEven {
    fn sub_assign(&mut self, other: CircleRotorAligningOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() - other.group0().with_w(other[e12345]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOriginAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        7        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        3        7        0
    //  no simd       12        7        0
    fn sub(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + Simd32x3::from(0.0).with_w(other[e12345] * -1.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<CircleRotorAligningOriginAtInfinity> for VersorEven {
    fn sub_assign(&mut self, other: CircleRotorAligningOriginAtInfinity) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + Simd32x3::from(0.0).with_w(other[e12345] * -1.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<CircleRotorAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        3        4        0
    //  no simd       12        4        0
    fn sub(self, other: CircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + Simd32x3::from(0.0).with_w(other[e12345] * -1.0),
            // e415, e425, e435, e321
            self.group1() - other.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<CircleRotorAtInfinity> for VersorEven {
    fn sub_assign(&mut self, other: CircleRotorAtInfinity) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + Simd32x3::from(0.0).with_w(other[e12345] * -1.0),
            // e415, e425, e435, e321
            self.group1() - other.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<CircleRotorOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        8        3        0
    fn sub(self, other: CircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() - other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<CircleRotorOnOrigin> for VersorEven {
    fn sub_assign(&mut self, other: CircleRotorOnOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() - other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<Dipole> for VersorEven {
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
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(other[e45]) * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group2() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleAligningOrigin> for VersorEven {
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
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleAtInfinity> for VersorEven {
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
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleAtOrigin> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleInversion> for VersorEven {
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
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(other[e45]) * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionAligningOrigin> for VersorEven {
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
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionAtInfinity> for VersorEven {
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
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionAtOrigin> for VersorEven {
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
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionOnOrigin> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            other.group1() * Simd32x4::from(-1.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionOrthogonalOrigin> for VersorEven {
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
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleOnOrigin> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleOrthogonalOrigin> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn sub(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            other.group2() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DualNum> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        8        2        0
    fn sub(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + Simd32x3::from(0.0).with_w(other[e12345] * -1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::SubAssign<DualNum> for VersorEven {
    fn sub_assign(&mut self, other: DualNum) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + Simd32x3::from(0.0).with_w(other[e12345] * -1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::Sub<FlatOrigin> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<FlatPoint> for VersorEven {
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
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<FlatPointAtInfinity> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: FlatPointAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Flector> for VersorEven {
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
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<FlectorAtInfinity> for VersorEven {
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
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<FlectorOnOrigin> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Horizon> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<Infinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2() + Simd32x3::from(0.0).with_w(other[e5] * -1.0),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<Infinity> for VersorEven {
    fn sub_assign(&mut self, other: Infinity) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2() + Simd32x3::from(0.0).with_w(other[e5] * -1.0),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<Line> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        8        6        0
    fn sub(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<Line> for VersorEven {
    fn sub_assign(&mut self, other: Line) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<LineAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        4        3        0
    fn sub(self, other: LineAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<LineAtInfinity> for VersorEven {
    fn sub_assign(&mut self, other: LineAtInfinity) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<LineOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        4        3        0
    fn sub(self, other: LineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<LineOnOrigin> for VersorEven {
    fn sub_assign(&mut self, other: LineOnOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<Motor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        3        4        0
    //  no simd       12        4        0
    fn sub(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + Simd32x3::from(0.0).with_w(other[e12345] * -1.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group1(),
            // e235, e315, e125, e5
            self.group2() - other.group1(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<Motor> for VersorEven {
    fn sub_assign(&mut self, other: Motor) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + Simd32x3::from(0.0).with_w(other[e12345] * -1.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group1(),
            // e235, e315, e125, e5
            self.group2() - other.group1(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<MotorAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: MotorAtInfinity) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2() - other.group0(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<MotorAtInfinity> for VersorEven {
    fn sub_assign(&mut self, other: MotorAtInfinity) {
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2() - other.group0(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<MotorOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        4        0
    fn sub(self, other: MotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + Simd32x3::from(0.0).with_w(other[e12345] * -1.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<MotorOnOrigin> for VersorEven {
    fn sub_assign(&mut self, other: MotorOnOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + Simd32x3::from(0.0).with_w(other[e12345] * -1.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<MultiVector> for VersorEven {
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
            Simd32x2::from([other[scalar], self[e12345] - other[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3() - other.group1(),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            other.group3() * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group4() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group5() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() - other.group6(),
            // e423, e431, e412
            self.group0().xyz() - other.group7(),
            // e235, e315, e125
            self.group2().xyz() - other.group8(),
            // e1234, e4235, e4315, e4125
            other.group9() * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<MysteryCircle> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: MysteryCircle) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1() - other.group0(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<MysteryCircle> for VersorEven {
    fn sub_assign(&mut self, other: MysteryCircle) {
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1() - other.group0(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<MysteryCircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        8        1        0
    fn sub(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + Simd32x3::from(0.0).with_w(other[e12345] * -1.0),
            // e415, e425, e435, e321
            self.group1() - other.group0(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<MysteryCircleRotor> for VersorEven {
    fn sub_assign(&mut self, other: MysteryCircleRotor) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + Simd32x3::from(0.0).with_w(other[e12345] * -1.0),
            // e415, e425, e435, e321
            self.group1() - other.group0(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<MysteryDipole> for VersorEven {
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
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MysteryDipoleInversion> for VersorEven {
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
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MysteryVersorEven> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        3        4        0
    //  no simd       12        4        0
    fn sub(self, other: MysteryVersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + Simd32x3::from(0.0).with_w(other[e12345] * -1.0),
            // e415, e425, e435, e321
            self.group1() - other.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group3(),
        );
    }
}
impl std::ops::SubAssign<MysteryVersorEven> for VersorEven {
    fn sub_assign(&mut self, other: MysteryVersorEven) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + Simd32x3::from(0.0).with_w(other[e12345] * -1.0),
            // e415, e425, e435, e321
            self.group1() - other.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group3(),
        );
    }
}
impl std::ops::Sub<MysteryVersorOdd> for VersorEven {
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
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullCircleAtOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        4        3        0
    fn sub(self, other: NullCircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<NullCircleAtOrigin> for VersorEven {
    fn sub_assign(&mut self, other: NullCircleAtOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<NullDipoleAtOrigin> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: NullDipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullDipoleInversionAtOrigin> for VersorEven {
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
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullSphereAtOrigin> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn sub(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullVersorEvenAtOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        4        0
    fn sub(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::SubAssign<NullVersorEvenAtOrigin> for VersorEven {
    fn sub_assign(&mut self, other: NullVersorEvenAtOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::Sub<Origin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::SubAssign<Origin> for VersorEven {
    fn sub_assign(&mut self, other: Origin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::Sub<Plane> for VersorEven {
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
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<PlaneOnOrigin> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<RoundPoint> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        8        1        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2() + Simd32x3::from(0.0).with_w(other[e5] * -1.0),
            // e1, e2, e3, e4
            self.group3() - other.group0(),
        );
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
            self.group2() + Simd32x3::from(0.0).with_w(other[e5] * -1.0),
            // e1, e2, e3, e4
            self.group3() - other.group0(),
        );
    }
}
impl std::ops::Sub<RoundPointAtOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        8        2        0
    fn sub(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2() + Simd32x3::from(0.0).with_w(other[e5] * -1.0),
            // e1, e2, e3, e4
            self.group3() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::SubAssign<RoundPointAtOrigin> for VersorEven {
    fn sub_assign(&mut self, other: RoundPointAtOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2() + Simd32x3::from(0.0).with_w(other[e5] * -1.0),
            // e1, e2, e3, e4
            self.group3() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::Sub<Scalar> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Sphere> for VersorEven {
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
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<SphereAtOrigin> for VersorEven {
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
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<SphereOnOrigin> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            other.group0().wxyz() * Simd32x4::from(-1.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<VersorEven> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        4        0        0
    // no simd       16        0        0
    fn sub(self, other: VersorEven) -> Self::Output {
        return VersorEven::from_groups(
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
impl std::ops::Sub<VersorEvenAligningOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        4        0        0
    // Totals...
    // yes simd        4        4        0
    //  no simd       16        4        0
    fn sub(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() - other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group1(),
            // e235, e315, e125, e5
            self.group2() - other.group2(),
            // e1, e2, e3, e4
            self.group3() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::SubAssign<VersorEvenAligningOrigin> for VersorEven {
    fn sub_assign(&mut self, other: VersorEvenAligningOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() - other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group1(),
            // e235, e315, e125, e5
            self.group2() - other.group2(),
            // e1, e2, e3, e4
            self.group3() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::Sub<VersorEvenAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        4        0        0
    // Totals...
    // yes simd        4        4        0
    //  no simd       16        4        0
    fn sub(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + Simd32x3::from(0.0).with_w(other[e12345] * -1.0),
            // e415, e425, e435, e321
            self.group1() - other.group1(),
            // e235, e315, e125, e5
            self.group2() - other.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group3(),
        );
    }
}
impl std::ops::SubAssign<VersorEvenAtInfinity> for VersorEven {
    fn sub_assign(&mut self, other: VersorEvenAtInfinity) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() + Simd32x3::from(0.0).with_w(other[e12345] * -1.0),
            // e415, e425, e435, e321
            self.group1() - other.group1(),
            // e235, e315, e125, e5
            self.group2() - other.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group3(),
        );
    }
}
impl std::ops::Sub<VersorEvenAtOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        3        4        0
    //  no simd       12        4        0
    fn sub(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2() - other.group1(),
            // e1, e2, e3, e4
            self.group3() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::SubAssign<VersorEvenAtOrigin> for VersorEven {
    fn sub_assign(&mut self, other: VersorEvenAtOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2() - other.group1(),
            // e1, e2, e3, e4
            self.group3() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::Sub<VersorEvenOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        3        4        0
    //  no simd       12        4        0
    fn sub(self, other: VersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() - other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::SubAssign<VersorEvenOnOrigin> for VersorEven {
    fn sub_assign(&mut self, other: VersorEvenOnOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() - other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::Sub<VersorEvenOrthogonalOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        4        0        0
    // Totals...
    // yes simd        4        4        0
    //  no simd       16        4        0
    fn sub(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435, e321
            self.group1() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125, e5
            self.group2() - other.group1(),
            // e1, e2, e3, e4
            self.group3() - other.group2(),
        );
    }
}
impl std::ops::SubAssign<VersorEvenOrthogonalOrigin> for VersorEven {
    fn sub_assign(&mut self, other: VersorEvenOrthogonalOrigin) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435, e321
            self.group1() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125, e5
            self.group2() - other.group1(),
            // e1, e2, e3, e4
            self.group3() - other.group2(),
        );
    }
}
impl std::ops::Sub<VersorOdd> for VersorEven {
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
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().xyz().with_w(other[e45]) * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<VersorOddAtInfinity> for VersorEven {
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
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            other.group0().yzw() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<VersorOddOrthogonalOrigin> for VersorEven {
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
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0().xyz(),
            // e235, e315, e125
            self.group2().xyz(),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            other[e3215] * -1.0,
        );
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
            let mut error = "Elements from MultiVector do not fit into VersorEven { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEven::from_groups(
            // e423, e431, e412, e12345
            multi_vector.group7().with_w(multi_vector[e12345]),
            // e415, e425, e435, e321
            multi_vector.group6(),
            // e235, e315, e125, e5
            multi_vector.group8().with_w(multi_vector[e5]),
            // e1, e2, e3, e4
            multi_vector.group1(),
        ));
    }
}
