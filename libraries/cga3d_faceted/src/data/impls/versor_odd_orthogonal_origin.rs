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
//  Average:         9      14       0
//  Maximum:       154     184       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         4       4       0
//  Average:        21      25       0
//  Maximum:       352     384       0
impl std::ops::Add<AntiCircleOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: AntiCircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            (other.group0() + self.group0().xyz()).with_w(self[scalar]),
            // e23, e31, e12, e3215
            (other.group1() + self.group1().xyz()).with_w(self[e3215]),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<AntiCircleOnOrigin> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: AntiCircleOnOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            (other.group0() + self.group0().xyz()).with_w(self[scalar]),
            // e23, e31, e12, e3215
            (other.group1() + self.group1().xyz()).with_w(self[e3215]),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiCircleRotor> for VersorOddOrthogonalOrigin {
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
            other.group1() + self.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            (other.group2().xyz() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn add(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() + other.group0().with_w(other[scalar]),
            // e23, e31, e12, e3215
            (other.group1() + self.group1().xyz()).with_w(self[e3215]),
            // e15, e25, e35, e1234
            (other.group2().xyz() + self.group2().xyz()).with_w(self[e1234]),
        );
    }
}
impl std::ops::AddAssign<AntiCircleRotorAligningOrigin> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: AntiCircleRotorAligningOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() + other.group0().with_w(other[scalar]),
            // e23, e31, e12, e3215
            (other.group1() + self.group1().xyz()).with_w(self[e3215]),
            // e15, e25, e35, e1234
            (other.group2().xyz() + self.group2().xyz()).with_w(self[e1234]),
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOriginAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0().xyz().with_w(other[scalar] + self[scalar]),
            // e23, e31, e12, e3215
            (other.group0() + self.group1().xyz()).with_w(self[e3215]),
            // e15, e25, e35, e1234
            (other.group1().xyz() + self.group2().xyz()).with_w(self[e1234]),
        );
    }
}
impl std::ops::AddAssign<AntiCircleRotorAligningOriginAtInfinity> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0().xyz().with_w(other[scalar] + self[scalar]),
            // e23, e31, e12, e3215
            (other.group0() + self.group1().xyz()).with_w(self[e3215]),
            // e15, e25, e35, e1234
            (other.group1().xyz() + self.group2().xyz()).with_w(self[e1234]),
        );
    }
}
impl std::ops::Add<AntiCircleRotorAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        8        0        0
    fn add(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().xyz().with_w(other[scalar] + self[scalar]),
            // e23, e31, e12, e45
            other.group0() + self.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            (other.group1().xyz() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
        );
    }
}
impl std::ops::Add<AntiCircleRotorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            other.group0() + self.group0(),
            // e23, e31, e12, e3215
            (other.group1() + self.group1().xyz()).with_w(self[e3215]),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<AntiCircleRotorOnOrigin> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: AntiCircleRotorOnOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            other.group0() + self.group0(),
            // e23, e31, e12, e3215
            (other.group1() + self.group1().xyz()).with_w(self[e3215]),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiDipoleInversion> for VersorOddOrthogonalOrigin {
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
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDipoleInversionAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            other.group2().xyz().with_w(0.0),
            // e5
            other[e5],
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            other.group1().yzwx(),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group1().with_w(0.0),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDipoleOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDualNum> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0().xyz().with_w(other[scalar] + self[scalar]),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(other[e1234] + self[e1234]),
        );
    }
}
impl std::ops::AddAssign<AntiDualNum> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: AntiDualNum) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0().xyz().with_w(other[scalar] + self[scalar]),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(other[e1234] + self[e1234]),
        );
    }
}
impl std::ops::Add<AntiFlatOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiFlatPoint> for VersorOddOrthogonalOrigin {
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
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiFlector> for VersorOddOrthogonalOrigin {
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
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiFlectorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiLine> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            (other.group0() + self.group1().xyz()).with_w(self[e3215]),
            // e15, e25, e35, e1234
            (other.group1() + self.group2().xyz()).with_w(self[e1234]),
        );
    }
}
impl std::ops::AddAssign<AntiLine> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: AntiLine) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            (other.group0() + self.group1().xyz()).with_w(self[e3215]),
            // e15, e25, e35, e1234
            (other.group1() + self.group2().xyz()).with_w(self[e1234]),
        );
    }
}
impl std::ops::Add<AntiLineOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: AntiLineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            (other.group0() + self.group1().xyz()).with_w(self[e3215]),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<AntiLineOnOrigin> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: AntiLineOnOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            (other.group0() + self.group1().xyz()).with_w(self[e3215]),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiMotor> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        8        0        0
    fn add(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0().xyz().with_w(other[scalar] + self[scalar]),
            // e23, e31, e12, e3215
            self.group1() + other.group0().xyz().with_w(other[e3215]),
            // e15, e25, e35, e1234
            (other.group1().xyz() + self.group2().xyz()).with_w(self[e1234]),
        );
    }
}
impl std::ops::AddAssign<AntiMotor> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: AntiMotor) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0().xyz().with_w(other[scalar] + self[scalar]),
            // e23, e31, e12, e3215
            self.group1() + other.group0().xyz().with_w(other[e3215]),
            // e15, e25, e35, e1234
            (other.group1().xyz() + self.group2().xyz()).with_w(self[e1234]),
        );
    }
}
impl std::ops::Add<AntiMotorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0().xyz().with_w(other[scalar] + self[scalar]),
            // e23, e31, e12, e3215
            (other.group0().xyz() + self.group1().xyz()).with_w(self[e3215]),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<AntiMotorOnOrigin> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: AntiMotorOnOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0().xyz().with_w(other[scalar] + self[scalar]),
            // e23, e31, e12, e3215
            (other.group0().xyz() + self.group1().xyz()).with_w(self[e3215]),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiMysteryCircleRotor> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() + Simd32x3::from(0.0).with_w(other[scalar]),
            // e23, e31, e12, e45
            other.group0() + self.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
        );
    }
}
impl std::ops::Add<AntiMysteryDipoleInversion> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            other.group1().with_w(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiPlane> for VersorOddOrthogonalOrigin {
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
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiPlaneOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiPlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            other.group0().with_w(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiScalar> for VersorOddOrthogonalOrigin {
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
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiSphereOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiSphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            other.group0(),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiVersorEvenOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        8        0        0
    fn add(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            other.group0() + self.group0(),
            // e23, e31, e12, e3215
            (other.group1().xyz() + self.group1().xyz()).with_w(self[e3215]),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(other[e1234] + self[e1234]),
        );
    }
}
impl std::ops::AddAssign<AntiVersorEvenOnOrigin> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: AntiVersorEvenOnOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            other.group0() + self.group0(),
            // e23, e31, e12, e3215
            (other.group1().xyz() + self.group1().xyz()).with_w(self[e3215]),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(other[e1234] + self[e1234]),
        );
    }
}
impl std::ops::Add<Circle> for VersorOddOrthogonalOrigin {
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
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group1().with_w(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group1().with_w(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotor> for VersorOddOrthogonalOrigin {
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
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotorAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group1().with_w(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotorAligningOriginAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group0().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotorAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group1().with_w(0.0),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Dipole> for VersorOddOrthogonalOrigin {
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
            other.group1() + self.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            (other.group2() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
        );
    }
}
impl std::ops::Add<DipoleAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (other.group0().xyz() + self.group0().xyz()).with_w(self[scalar]),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(other[e45]),
            // e15, e25, e35, e1234
            (other.group1() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
        );
    }
}
impl std::ops::Add<DipoleAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            other.group0() + self.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            (other.group1() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
        );
    }
}
impl std::ops::Add<DipoleAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            (other.group0() + self.group0().xyz()).with_w(self[scalar]),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            (other.group1() + self.group2().xyz()).with_w(self[e1234]),
        );
    }
}
impl std::ops::AddAssign<DipoleAtOrigin> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: DipoleAtOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            (other.group0() + self.group0().xyz()).with_w(self[scalar]),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            (other.group1() + self.group2().xyz()).with_w(self[e1234]),
        );
    }
}
impl std::ops::Add<DipoleInversion> for VersorOddOrthogonalOrigin {
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
            other.group1() + self.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            other.group2() + self.group2(),
            // e4235, e4315, e4125, e3215
            other.group3() + Simd32x3::from(0.0).with_w(self[e3215]),
        );
    }
}
impl std::ops::Add<DipoleInversionAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       11        0        0
    fn add(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (other.group0().xyz() + self.group0().xyz()).with_w(self[scalar]),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(other[e45]),
            // e15, e25, e35, e1234
            other.group1() + self.group2(),
            // e4235, e4315, e4125, e3215
            other.group2() + Simd32x3::from(0.0).with_w(self[e3215]),
        );
    }
}
impl std::ops::Add<DipoleInversionAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       11        0        0
    fn add(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            other.group0() + self.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            (other.group1() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            other.group2() + Simd32x3::from(0.0).with_w(self[e3215]),
        );
    }
}
impl std::ops::Add<DipoleInversionAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        8        0        0
    fn add(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            (other.group0().xyz() + self.group0().xyz()).with_w(self[scalar]),
            // e23, e31, e12, e3215
            self.group1().xyz().with_w(other[e3215] + self[e3215]),
            // e15, e25, e35, e1234
            other.group1() + self.group2(),
        );
    }
}
impl std::ops::AddAssign<DipoleInversionAtOrigin> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: DipoleInversionAtOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            (other.group0().xyz() + self.group0().xyz()).with_w(self[scalar]),
            // e23, e31, e12, e3215
            self.group1().xyz().with_w(other[e3215] + self[e3215]),
            // e15, e25, e35, e1234
            other.group1() + self.group2(),
        );
    }
}
impl std::ops::Add<DipoleInversionOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (other.group0().xyz() + self.group0().xyz()).with_w(self[scalar]),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(other[e45]),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(other[e1234] + self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Add<DipoleInversionOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       11        0        0
    fn add(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            (other.group0().xyz() + self.group0().xyz()).with_w(self[scalar]),
            // e23, e31, e12, e3215
            self.group1() + other.group1().with_w(other[e3215]),
            // e15, e25, e35, e1234
            other.group2() + self.group2(),
        );
    }
}
impl std::ops::AddAssign<DipoleInversionOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: DipoleInversionOrthogonalOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            (other.group0().xyz() + self.group0().xyz()).with_w(self[scalar]),
            // e23, e31, e12, e3215
            self.group1() + other.group1().with_w(other[e3215]),
            // e15, e25, e35, e1234
            other.group2() + self.group2(),
        );
    }
}
impl std::ops::Add<DipoleOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (other.group0().xyz() + self.group0().xyz()).with_w(self[scalar]),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(other[e45]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
        );
    }
}
impl std::ops::Add<DipoleOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        3        0        0
    // no simd        9        0        0
    fn add(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            (other.group0() + self.group0().xyz()).with_w(self[scalar]),
            // e23, e31, e12, e3215
            (other.group1() + self.group1().xyz()).with_w(self[e3215]),
            // e15, e25, e35, e1234
            (other.group2() + self.group2().xyz()).with_w(self[e1234]),
        );
    }
}
impl std::ops::AddAssign<DipoleOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: DipoleOrthogonalOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            (other.group0() + self.group0().xyz()).with_w(self[scalar]),
            // e23, e31, e12, e3215
            (other.group1() + self.group1().xyz()).with_w(self[e3215]),
            // e15, e25, e35, e1234
            (other.group2() + self.group2().xyz()).with_w(self[e1234]),
        );
    }
}
impl std::ops::Add<DualNum> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<FlatOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    fn add(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(other[e45]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
        );
    }
}
impl std::ops::Add<FlatPoint> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(other[e45]),
            // e15, e25, e35, e1234
            (other.group0().xyz() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
        );
    }
}
impl std::ops::Add<FlatPointAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: FlatPointAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            (other.group0() + self.group2().xyz()).with_w(self[e1234]),
        );
    }
}
impl std::ops::AddAssign<FlatPointAtInfinity> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: FlatPointAtInfinity) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            (other.group0() + self.group2().xyz()).with_w(self[e1234]),
        );
    }
}
impl std::ops::Add<Flector> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(other[e45]),
            // e15, e25, e35, e1234
            (other.group0().xyz() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            other.group1() + Simd32x3::from(0.0).with_w(self[e3215]),
        );
    }
}
impl std::ops::Add<FlectorAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            self.group1().xyz().with_w(other[e3215] + self[e3215]),
            // e15, e25, e35, e1234
            (other.group0().xyz() + self.group2().xyz()).with_w(self[e1234]),
        );
    }
}
impl std::ops::AddAssign<FlectorAtInfinity> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: FlectorAtInfinity) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            self.group1().xyz().with_w(other[e3215] + self[e3215]),
            // e15, e25, e35, e1234
            (other.group0().xyz() + self.group2().xyz()).with_w(self[e1234]),
        );
    }
}
impl std::ops::Add<FlectorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    fn add(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(other[e45]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Add<Horizon> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            self.group1() + Simd32x3::from(0.0).with_w(other[e3215]),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<Horizon> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: Horizon) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            self.group1() + Simd32x3::from(0.0).with_w(other[e3215]),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::Add<Infinity> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5],
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Line> for VersorOddOrthogonalOrigin {
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
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group0().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<LineAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: LineAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<LineOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: LineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group0().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Motor> for VersorOddOrthogonalOrigin {
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
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MotorAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: MotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5],
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MotorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: MotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MultiVector> for VersorOddOrthogonalOrigin {
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
            // e41, e42, e43, e45
            other.group3() + self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            other.group4() + self.group2().xyz(),
            // e23, e31, e12
            other.group5() + self.group1().xyz(),
            // e415, e425, e435, e321
            other.group6(),
            // e423, e431, e412
            other.group7(),
            // e235, e315, e125
            other.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]) + other.group9(),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::Add<MysteryCircle> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: MysteryCircle) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MysteryCircleRotor> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MysteryDipole> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            other.group0() + self.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
        );
    }
}
impl std::ops::Add<MysteryDipoleInversion> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            other.group0() + self.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            other.group1().with_w(self[e3215]),
        );
    }
}
impl std::ops::Add<MysteryVersorEven> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: MysteryVersorEven) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MysteryVersorOdd> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().xyz().with_w(other[scalar] + self[scalar]),
            // e23, e31, e12, e45
            other.group1() + self.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Add<NullCircleAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: NullCircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<NullDipoleAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: NullDipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            (other.group0() + self.group0().xyz()).with_w(self[scalar]),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<NullDipoleAtOrigin> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: NullDipoleAtOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            (other.group0() + self.group0().xyz()).with_w(self[scalar]),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::Add<NullDipoleInversionAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            (other.group0().xyz() + self.group0().xyz()).with_w(self[scalar]),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(other[e1234] + self[e1234]),
        );
    }
}
impl std::ops::AddAssign<NullDipoleInversionAtOrigin> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: NullDipoleInversionAtOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            (other.group0().xyz() + self.group0().xyz()).with_w(self[scalar]),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(other[e1234] + self[e1234]),
        );
    }
}
impl std::ops::Add<NullSphereAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234]),
        );
    }
}
impl std::ops::AddAssign<NullSphereAtOrigin> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: NullSphereAtOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234]),
        );
    }
}
impl std::ops::Add<NullVersorEvenAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Origin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Plane> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            other.group0() + Simd32x3::from(0.0).with_w(self[e3215]),
        );
    }
}
impl std::ops::Add<PlaneOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    fn add(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            other.group0().with_w(self[e3215]),
        );
    }
}
impl std::ops::Add<RoundPoint> for VersorOddOrthogonalOrigin {
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
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<RoundPointAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Scalar> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() + Simd32x3::from(0.0).with_w(other[scalar]),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<Scalar> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() + Simd32x3::from(0.0).with_w(other[scalar]),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::Add<Sphere> for VersorOddOrthogonalOrigin {
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
            self.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234]),
            // e4235, e4315, e4125, e3215
            other.group0() + Simd32x3::from(0.0).with_w(self[e3215]),
        );
    }
}
impl std::ops::Add<SphereAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            self.group1().xyz().with_w(other[e3215] + self[e3215]),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(other[e1234] + self[e1234]),
        );
    }
}
impl std::ops::AddAssign<SphereAtOrigin> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: SphereAtOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            self.group1().xyz().with_w(other[e3215] + self[e3215]),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(other[e1234] + self[e1234]),
        );
    }
}
impl std::ops::Add<SphereOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(other[e1234] + self[e1234]),
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(self[e3215]),
        );
    }
}
impl std::ops::Add<VersorEven> for VersorOddOrthogonalOrigin {
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
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group1().xyz().with_w(0.0),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group1().xyz().with_w(0.0),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            other.group2(),
            // e5
            other[e5],
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorOdd> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        4        0        0
    // no simd       16        0        0
    fn add(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() + self.group0(),
            // e23, e31, e12, e45
            other.group1() + self.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            other.group2() + self.group2(),
            // e4235, e4315, e4125, e3215
            other.group3() + Simd32x3::from(0.0).with_w(self[e3215]),
        );
    }
}
impl std::ops::Add<VersorOddAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       12        0        0
    fn add(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().xyz().with_w(other[scalar] + self[scalar]),
            // e23, e31, e12, e45
            other.group1() + self.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            (self.group2().xyz() + other.group0().yzw()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            other.group2() + Simd32x3::from(0.0).with_w(self[e3215]),
        );
    }
}
impl std::ops::Add<VersorOddOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        3        0        0
    // no simd       12        0        0
    fn add(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            other.group0() + self.group0(),
            // e23, e31, e12, e3215
            other.group1() + self.group1(),
            // e15, e25, e35, e1234
            other.group2() + self.group2(),
        );
    }
}
impl std::ops::AddAssign<VersorOddOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: VersorOddOrthogonalOrigin) {
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            other.group0() + self.group0(),
            // e23, e31, e12, e3215
            other.group1() + self.group1(),
            // e15, e25, e35, e1234
            other.group2() + self.group2(),
        );
    }
}

impl From<AntiCircleOnOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_anti_circle_on_origin: AntiCircleOnOrigin) -> Self {
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            from_anti_circle_on_origin.group0().with_w(0.0),
            // e23, e31, e12, e3215
            from_anti_circle_on_origin.group1().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiCircleRotorAligningOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_anti_circle_rotor_aligning_origin: AntiCircleRotorAligningOrigin) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            from_anti_circle_rotor_aligning_origin.group0().with_w(from_anti_circle_rotor_aligning_origin[scalar]),
            // e23, e31, e12, e3215
            from_anti_circle_rotor_aligning_origin.group1().with_w(0.0),
            // e15, e25, e35, e1234
            from_anti_circle_rotor_aligning_origin.group2().xyz().with_w(0.0),
        );
    }
}

impl From<AntiCircleRotorAligningOriginAtInfinity> for VersorOddOrthogonalOrigin {
    fn from(from_anti_circle_rotor_aligning_origin_at_infinity: AntiCircleRotorAligningOriginAtInfinity) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(from_anti_circle_rotor_aligning_origin_at_infinity[scalar]),
            // e23, e31, e12, e3215
            from_anti_circle_rotor_aligning_origin_at_infinity.group0().with_w(0.0),
            // e15, e25, e35, e1234
            from_anti_circle_rotor_aligning_origin_at_infinity.group1().xyz().with_w(0.0),
        );
    }
}

impl From<AntiCircleRotorOnOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_anti_circle_rotor_on_origin: AntiCircleRotorOnOrigin) -> Self {
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            from_anti_circle_rotor_on_origin.group0(),
            // e23, e31, e12, e3215
            from_anti_circle_rotor_on_origin.group1().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiDualNum> for VersorOddOrthogonalOrigin {
    fn from(from_anti_dual_num: AntiDualNum) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(from_anti_dual_num[scalar]),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_anti_dual_num[e1234]),
        );
    }
}

impl From<AntiLine> for VersorOddOrthogonalOrigin {
    fn from(from_anti_line: AntiLine) -> Self {
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            from_anti_line.group0().with_w(0.0),
            // e15, e25, e35, e1234
            from_anti_line.group1().with_w(0.0),
        );
    }
}

impl From<AntiLineOnOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_anti_line_on_origin: AntiLineOnOrigin) -> Self {
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            from_anti_line_on_origin.group0().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiMotor> for VersorOddOrthogonalOrigin {
    fn from(from_anti_motor: AntiMotor) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(from_anti_motor[scalar]),
            // e23, e31, e12, e3215
            Simd32x4::from([from_anti_motor[e23], from_anti_motor[e31], from_anti_motor[e12], from_anti_motor[e3215]]),
            // e15, e25, e35, e1234
            from_anti_motor.group1().xyz().with_w(0.0),
        );
    }
}

impl From<AntiMotorOnOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_anti_motor_on_origin: AntiMotorOnOrigin) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(from_anti_motor_on_origin[scalar]),
            // e23, e31, e12, e3215
            from_anti_motor_on_origin.group0().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiVersorEvenOnOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_anti_versor_even_on_origin: AntiVersorEvenOnOrigin) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            from_anti_versor_even_on_origin.group0(),
            // e23, e31, e12, e3215
            from_anti_versor_even_on_origin.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_anti_versor_even_on_origin[e1234]),
        );
    }
}

impl From<DipoleAtOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_dipole_at_origin: DipoleAtOrigin) -> Self {
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            from_dipole_at_origin.group0().with_w(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            from_dipole_at_origin.group1().with_w(0.0),
        );
    }
}

impl From<DipoleInversionAtOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_dipole_inversion_at_origin: DipoleInversionAtOrigin) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            from_dipole_inversion_at_origin.group0().xyz().with_w(0.0),
            // e23, e31, e12, e3215
            Simd32x3::from(0.0).with_w(from_dipole_inversion_at_origin[e3215]),
            // e15, e25, e35, e1234
            from_dipole_inversion_at_origin.group1(),
        );
    }
}

impl From<DipoleInversionOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_dipole_inversion_orthogonal_origin: DipoleInversionOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            from_dipole_inversion_orthogonal_origin.group0().xyz().with_w(0.0),
            // e23, e31, e12, e3215
            from_dipole_inversion_orthogonal_origin.group1().with_w(from_dipole_inversion_orthogonal_origin[e3215]),
            // e15, e25, e35, e1234
            from_dipole_inversion_orthogonal_origin.group2(),
        );
    }
}

impl From<DipoleOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_dipole_orthogonal_origin: DipoleOrthogonalOrigin) -> Self {
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            from_dipole_orthogonal_origin.group0().with_w(0.0),
            // e23, e31, e12, e3215
            from_dipole_orthogonal_origin.group1().with_w(0.0),
            // e15, e25, e35, e1234
            from_dipole_orthogonal_origin.group2().with_w(0.0),
        );
    }
}

impl From<FlatPointAtInfinity> for VersorOddOrthogonalOrigin {
    fn from(from_flat_point_at_infinity: FlatPointAtInfinity) -> Self {
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            from_flat_point_at_infinity.group0().with_w(0.0),
        );
    }
}

impl From<FlectorAtInfinity> for VersorOddOrthogonalOrigin {
    fn from(from_flector_at_infinity: FlectorAtInfinity) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x3::from(0.0).with_w(from_flector_at_infinity[e3215]),
            // e15, e25, e35, e1234
            from_flector_at_infinity.group0().xyz().with_w(0.0),
        );
    }
}

impl From<Horizon> for VersorOddOrthogonalOrigin {
    fn from(from_horizon: Horizon) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x3::from(0.0).with_w(from_horizon[e3215]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        );
    }
}

impl From<NullDipoleAtOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_null_dipole_at_origin: NullDipoleAtOrigin) -> Self {
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            from_null_dipole_at_origin.group0().with_w(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        );
    }
}

impl From<NullDipoleInversionAtOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_null_dipole_inversion_at_origin: NullDipoleInversionAtOrigin) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            from_null_dipole_inversion_at_origin.group0().xyz().with_w(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_null_dipole_inversion_at_origin[e1234]),
        );
    }
}

impl From<NullSphereAtOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_null_sphere_at_origin: NullSphereAtOrigin) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_null_sphere_at_origin[e1234]),
        );
    }
}

impl From<Scalar> for VersorOddOrthogonalOrigin {
    fn from(from_scalar: Scalar) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(from_scalar[scalar]),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        );
    }
}

impl From<SphereAtOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_sphere_at_origin: SphereAtOrigin) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x3::from(0.0).with_w(from_sphere_at_origin[e3215]),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_sphere_at_origin[e1234]),
        );
    }
}
impl std::ops::Mul<AntiCircleOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       53        0
    //    simd3        0        4        0
    //    simd4        6        2        0
    // Totals...
    // yes simd       38       59        0
    //  no simd       56       73        0
    fn mul(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       62        0
    //    simd3        0        6        0
    //    simd4       19       13        0
    // Totals...
    // yes simd       60       81        0
    //  no simd      117      132        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       61        0
    //    simd3        0        5        0
    //    simd4       16       11        0
    // Totals...
    // yes simd       57       77        0
    //  no simd      105      120        0
    fn mul(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOriginAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       44        0
    //    simd3        0        3        0
    //    simd4       11        8        0
    // Totals...
    // yes simd       39       55        0
    //  no simd       72       85        0
    fn mul(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       41        0
    //    simd3        0        1        0
    //    simd4       14       13        0
    // Totals...
    // yes simd       40       55        0
    //  no simd       82       96        0
    fn mul(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       52        0
    //    simd3        0        3        0
    //    simd4        9        6        0
    // Totals...
    // yes simd       41       61        0
    //  no simd       68       85        0
    fn mul(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       75        0
    //    simd3        0       11        0
    //    simd4       29       18        0
    // Totals...
    // yes simd       77      104        0
    //  no simd      164      180        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       61        0
    //    simd3        0        5        0
    //    simd4       19       14        0
    // Totals...
    // yes simd       59       80        0
    //  no simd      116      132        0
    fn mul(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       29        0
    //    simd3        0        5        0
    //    simd4       17       13        0
    // Totals...
    // yes simd       33       47        0
    //  no simd       84       96        0
    fn mul(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       63        0
    //    simd3        0        6        0
    //    simd4       19       13        0
    // Totals...
    // yes simd       59       82        0
    //  no simd      116      133        0
    fn mul(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       24        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       14       31        0
    //  no simd       32       52        0
    fn mul(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        1        2        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        3        9        0
    //  no simd        8       25        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        5        0
    // no simd        0       20        0
    fn mul(self, other: AntiFlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        0
    //    simd3        3        6        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       14       28        0
    //  no simd       32       52        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       38        0
    //    simd3        0        2        0
    //    simd4       14       13        0
    // Totals...
    // yes simd       39       53        0
    //  no simd       81       96        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlectorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
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
impl std::ops::Mul<AntiLine> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       44        0
    //    simd3        0        3        0
    //    simd4        8        5        0
    // Totals...
    // yes simd       33       52        0
    //  no simd       57       73        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLineOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       27        0
    //    simd3        0        3        0
    //    simd4        3        0        0
    // Totals...
    // yes simd       15       30        0
    //  no simd       24       36        0
    fn mul(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiLineOnOrigin> for VersorOddOrthogonalOrigin {
    fn mul_assign(&mut self, other: AntiLineOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       46        0
    //    simd3        0        5        0
    //    simd4       14        9        0
    // Totals...
    // yes simd       42       60        0
    //  no simd       84       97        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
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
impl std::ops::MulAssign<AntiMotorOnOrigin> for VersorOddOrthogonalOrigin {
    fn mul_assign(&mut self, other: AntiMotorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryCircleRotor> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       22        0
    //    simd3        3        5        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       20       33        0
    //  no simd       44       61        0
    fn mul(self, other: AntiMysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryDipoleInversion> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       38        0
    //    simd3        0        2        0
    //    simd4       12       10        0
    // Totals...
    // yes simd       36       50        0
    //  no simd       72       84        0
    fn mul(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       19        0
    //    simd3        2        4        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       14       28        0
    //  no simd       33       51        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlaneOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       20        0
    //    simd4        5        5        0
    // Totals...
    // yes simd        9       25        0
    //  no simd       24       40        0
    fn mul(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for VersorOddOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiSphereOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       21        0
    //    simd3        3        5        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       17       29        0
    //  no simd       32       48        0
    fn mul(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorEvenOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       52        0
    //    simd3        0        3        0
    //    simd4       12        9        0
    // Totals...
    // yes simd       44       64        0
    //  no simd       80       97        0
    fn mul(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       63        0
    //    simd3        0        7        0
    //    simd4       16        9        0
    // Totals...
    // yes simd       57       79        0
    //  no simd      105      120        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       62        0
    //    simd3        0        6        0
    //    simd4       13        7        0
    // Totals...
    // yes simd       54       75        0
    //  no simd       93      108        0
    fn mul(self, other: CircleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       51        0
    //    simd3        0        2        0
    //    simd4        9        7        0
    // Totals...
    // yes simd       41       60        0
    //  no simd       68       85        0
    fn mul(self, other: CircleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       36        0
    //    simd3        0        3        0
    //    simd4       10        7        0
    // Totals...
    // yes simd       27       46        0
    //  no simd       57       73        0
    fn mul(self, other: CircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       42        0
    //    simd3        0        2        0
    //    simd4        8        6        0
    // Totals...
    // yes simd       33       50        0
    //  no simd       57       72        0
    fn mul(self, other: CircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       37        0
    //    simd3        0        3        0
    //    simd4       13       10        0
    // Totals...
    // yes simd       32       50        0
    //  no simd       71       86        0
    fn mul(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       63        0
    //    simd3        0        7        0
    //    simd4       19       12        0
    // Totals...
    // yes simd       60       82        0
    //  no simd      117      132        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       62        0
    //    simd3        0        6        0
    //    simd4       16       10        0
    // Totals...
    // yes simd       57       78        0
    //  no simd      105      120        0
    fn mul(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOriginAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       52        0
    //    simd3        0        3        0
    //    simd4        9        6        0
    // Totals...
    // yes simd       41       61        0
    //  no simd       68       85        0
    fn mul(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       52        0
    //    simd3        0        3        0
    //    simd4       12        9        0
    // Totals...
    // yes simd       44       64        0
    //  no simd       80       97        0
    fn mul(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       41        0
    //    simd3        0        1        0
    //    simd4       11       10        0
    // Totals...
    // yes simd       39       52        0
    //  no simd       72       84        0
    fn mul(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       63        0
    //    simd3        0        7        0
    //    simd4       16        9        0
    // Totals...
    // yes simd       57       79        0
    //  no simd      105      120        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       38        0
    //    simd3        0        3        0
    //    simd4       13       10        0
    // Totals...
    // yes simd       32       51        0
    //  no simd       71       87        0
    fn mul(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       41        0
    //    simd3        0        1        0
    //    simd4       11       10        0
    // Totals...
    // yes simd       37       52        0
    //  no simd       70       84        0
    fn mul(self, other: DipoleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       36        0
    //    simd3        0        3        0
    //    simd4       10        7        0
    // Totals...
    // yes simd       27       46        0
    //  no simd       57       73        0
    fn mul(self, other: DipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       73        0
    //    simd3        0        9        0
    //    simd4       29       20        0
    // Totals...
    // yes simd       77      102        0
    //  no simd      164      180        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       47        0
    //    simd3        0        3        0
    //    simd4       25       22        0
    // Totals...
    // yes simd       54       72        0
    //  no simd      129      144        0
    fn mul(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       46        0
    //    simd3        0        5        0
    //    simd4       23       18        0
    // Totals...
    // yes simd       50       69        0
    //  no simd      119      133        0
    fn mul(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       40        0
    //    simd3        0        4        0
    //    simd4       15       11        0
    // Totals...
    // yes simd       36       55        0
    //  no simd       81       96        0
    fn mul(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       43        0
    //    simd3        0        2        0
    //    simd4       14       12        0
    // Totals...
    // yes simd       42       57        0
    //  no simd       84       97        0
    fn mul(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       63        0
    //    simd3        0        6        0
    //    simd4       19       13        0
    // Totals...
    // yes simd       59       82        0
    //  no simd      116      133        0
    fn mul(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       22        0
    //    simd3        3        6        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       17       31        0
    //  no simd       32       52        0
    fn mul(self, other: DipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       61        0
    //    simd3        0        5        0
    //    simd4       13        8        0
    // Totals...
    // yes simd       54       74        0
    //  no simd       93      108        0
    fn mul(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        4        0
    //    simd3        1        2        0
    //    simd4        1        5        0
    // Totals...
    // yes simd        3       11        0
    //  no simd        8       30        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatOrigin> for VersorOddOrthogonalOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        5        0
    // no simd        0       20        0
    fn mul(self, other: FlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       20        0
    //    simd3        3        5        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       14       29        0
    //  no simd       32       51        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPointAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       20        0
    //    simd4        5        5        0
    // Totals...
    // yes simd        9       25        0
    //  no simd       24       40        0
    fn mul(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       31        0
    //    simd3        0        7        0
    //    simd4       17       11        0
    // Totals...
    // yes simd       33       49        0
    //  no simd       84       96        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        8       10        0
    // Totals...
    // yes simd       12       18        0
    //  no simd       36       48        0
    fn mul(self, other: FlectorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = DipoleInversionAligningOrigin;
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
impl std::ops::Mul<Horizon> for VersorOddOrthogonalOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        5        0
    // no simd        0       20        0
    fn mul(self, other: Horizon) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Infinity> for VersorOddOrthogonalOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn mul(self, other: Infinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       52        0
    //    simd3        0        3        0
    //    simd4        6        3        0
    // Totals...
    // yes simd       38       58        0
    //  no simd       56       73        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       15       27        0
    //  no simd       24       36        0
    fn mul(self, other: LineAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       26        0
    //    simd3        0        2        0
    //    simd4        3        1        0
    // Totals...
    // yes simd       15       29        0
    //  no simd       24       36        0
    fn mul(self, other: LineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       51        0
    //    simd3        0        2        0
    //    simd4       12       10        0
    // Totals...
    // yes simd       44       63        0
    //  no simd       80       97        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       18       30        0
    //  no simd       36       48        0
    fn mul(self, other: MotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       18       30        0
    //  no simd       36       48        0
    fn mul(self, other: MotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       66       91        0
    //    simd2       11       12        0
    //    simd3       44       55        0
    //    simd4       33       26        0
    // Totals...
    // yes simd      154      184        0
    //  no simd      352      384        0
    fn mul(self, other: MultiVector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircle> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       21        0
    //    simd3        2        4        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       16       29        0
    //  no simd       32       49        0
    fn mul(self, other: MysteryCircle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircleRotor> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       22        0
    //    simd3        3        5        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       20       33        0
    //  no simd       44       61        0
    fn mul(self, other: MysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipole> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       21        0
    //    simd3        2        4        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       16       29        0
    //  no simd       32       49        0
    fn mul(self, other: MysteryDipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipoleInversion> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       37        0
    //    simd3        0        4        0
    //    simd4       13        9        0
    // Totals...
    // yes simd       32       50        0
    //  no simd       71       85        0
    fn mul(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorEven> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       43        0
    //    simd3        0        3        0
    //    simd4       14       11        0
    // Totals...
    // yes simd       42       57        0
    //  no simd       84       96        0
    fn mul(self, other: MysteryVersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorOdd> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       42        0
    //    simd3        0        5        0
    //    simd4       15       10        0
    // Totals...
    // yes simd       38       57        0
    //  no simd       83       97        0
    fn mul(self, other: MysteryVersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullCircleAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       21        0
    //    simd3        0        1        0
    //    simd4        5        4        0
    // Totals...
    // yes simd        9       26        0
    //  no simd       24       40        0
    fn mul(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       20        0
    //    simd3        0        4        0
    //    simd4        4        1        0
    // Totals...
    // yes simd       14       25        0
    //  no simd       26       36        0
    fn mul(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleInversionAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       20        0
    //    simd3        0        4        0
    //    simd4        7        4        0
    // Totals...
    // yes simd       18       28        0
    //  no simd       39       48        0
    fn mul(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullSphereAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn mul(self, other: NullSphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullVersorEvenAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        8       10        0
    // Totals...
    // yes simd       12       18        0
    //  no simd       36       48        0
    fn mul(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Origin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        5        0
    // no simd        0       20        0
    fn mul(self, other: Origin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       21        0
    //    simd3        2        4        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       16       29        0
    //  no simd       32       49        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<PlaneOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       21        0
    //    simd3        0        1        0
    //    simd4        5        4        0
    // Totals...
    // yes simd        9       26        0
    //  no simd       24       40        0
    fn mul(self, other: PlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       31        0
    //    simd3        0        3        0
    //    simd4        8        5        0
    // Totals...
    // yes simd       21       39        0
    //  no simd       45       60        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPointAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        8        0
    //    simd3        1        2        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        3       13        0
    //  no simd        8       26        0
    fn mul(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn mul(self, other: Scalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for VersorOddOrthogonalOrigin {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       28        0
    //    simd3        0        3        0
    //    simd4        9        6        0
    // Totals...
    // yes simd       18       37        0
    //  no simd       45       61        0
    fn mul(self, other: Sphere) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        8        0
    //    simd3        1        2        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        3       13        0
    //  no simd        8       26        0
    fn mul(self, other: SphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       13        0
    //    simd3        3        5        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       12       24        0
    //  no simd       36       52        0
    fn mul(self, other: SphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       74        0
    //    simd3        0       10        0
    //    simd4       32       22        0
    // Totals...
    // yes simd       80      106        0
    //  no simd      176      192        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       63        0
    //    simd3        0        6        0
    //    simd4       22       16        0
    // Totals...
    // yes simd       62       85        0
    //  no simd      128      145        0
    fn mul(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       62        0
    //    simd3        0        6        0
    //    simd4       22       16        0
    // Totals...
    // yes simd       62       84        0
    //  no simd      128      144        0
    fn mul(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       36        0
    //    simd4       15       15        0
    // Totals...
    // yes simd       36       51        0
    //  no simd       81       96        0
    fn mul(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       43        0
    //    simd3        0        3        0
    //    simd4       14       11        0
    // Totals...
    // yes simd       42       57        0
    //  no simd       84       96        0
    fn mul(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       50        0
    //    simd3        0        6        0
    //    simd4       25       19        0
    // Totals...
    // yes simd       54       75        0
    //  no simd      129      144        0
    fn mul(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       76        0
    //    simd3        0       12        0
    //    simd4       32       20        0
    // Totals...
    // yes simd       80      108        0
    //  no simd      176      192        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       49        0
    //    simd3        0        4        0
    //    simd4       25       21        0
    // Totals...
    // yes simd       56       74        0
    //  no simd      131      145        0
    fn mul(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       63        0
    //    simd3        0        6        0
    //    simd4       22       16        0
    // Totals...
    // yes simd       62       85        0
    //  no simd      128      145        0
    fn mul(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Neg for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn neg(self) -> Self::Output {
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12, e3215
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Not for VersorOddOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiCircleOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        8        6        0
    fn sub(self, other: AntiCircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiCircleOnOrigin> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: AntiCircleOnOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiCircleRotor> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        1        0        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        3        4        0
    //  no simd       11        7        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() - other.group0().with_w(other[scalar]),
            // e23, e31, e12, e45
            (self.group1().xyz() - other.group1().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        3        6        0
    //  no simd       12        6        0
    fn sub(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() - other.group0().with_w(other[scalar]),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiCircleRotorAligningOrigin> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: AntiCircleRotorAligningOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() - other.group0().with_w(other[scalar]),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOriginAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        7        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        3        7        0
    //  no simd       12        7        0
    fn sub(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiCircleRotorAligningOriginAtInfinity> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        1        0        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd       11        8        0
    fn sub(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
            // e23, e31, e12, e45
            (self.group1().xyz() - other.group0().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        8        3        0
    fn sub(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() - other.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiCircleRotorOnOrigin> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: AntiCircleRotorOnOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() - other.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversion> for VersorOddOrthogonalOrigin {
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
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       11        0
    fn sub(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            (other.group2().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn sub(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            other.group1().yzwx() * Simd32x4::from(-1.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       14        0
    fn sub(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            (other.group1() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDipoleOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: AntiDipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDualNum> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        8        2        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
        );
    }
}
impl std::ops::SubAssign<AntiDualNum> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: AntiDualNum) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
        );
    }
}
impl std::ops::Sub<AntiFlatOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiFlatPoint> for VersorOddOrthogonalOrigin {
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
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiFlector> for VersorOddOrthogonalOrigin {
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
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiFlectorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            (other.group0().yzw() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiLine> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        8        6        0
    fn sub(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiLine> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: AntiLine) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<AntiLineOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        4        3        0
    fn sub(self, other: AntiLineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiLineOnOrigin> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: AntiLineOnOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiMotor> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        3        4        0
    //  no simd       12        4        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
            // e23, e31, e12, e3215
            self.group1() - other.group0().xyz().with_w(other[e3215]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiMotor> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: AntiMotor) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
            // e23, e31, e12, e3215
            self.group1() - other.group0().xyz().with_w(other[e3215]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<AntiMotorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        4        0
    fn sub(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiMotorOnOrigin> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: AntiMotorOnOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiMysteryCircleRotor> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        0        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        7        5        0
    fn sub(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
            // e23, e31, e12, e45
            (self.group1().xyz() - other.group0().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
        );
    }
}
impl std::ops::Sub<AntiMysteryDipoleInversion> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            (other.group1() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiPlane> for VersorOddOrthogonalOrigin {
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
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiPlaneOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: AntiPlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiScalar> for VersorOddOrthogonalOrigin {
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
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiSphereOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiSphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            other.group0() * Simd32x4::from(-1.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiVersorEvenOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        3        4        0
    //  no simd       12        4        0
    fn sub(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() - other.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
        );
    }
}
impl std::ops::SubAssign<AntiVersorEvenOnOrigin> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: AntiVersorEvenOnOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() - other.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
        );
    }
}
impl std::ops::Sub<Circle> for VersorOddOrthogonalOrigin {
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
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn sub(self, other: CircleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            (other.group1() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: CircleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: CircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: CircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            (other.group1() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn sub(self, other: CircleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotor> for VersorOddOrthogonalOrigin {
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
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       11        0
    fn sub(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            (other.group1() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOriginAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        8        0
    fn sub(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotorAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn sub(self, other: CircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        8        0
    fn sub(self, other: CircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            (other.group1() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Dipole> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        1        0        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        3        7        0
    //  no simd       11       10        0
    fn sub(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0(),
            // e23, e31, e12, e45
            (self.group1().xyz() - other.group1().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
        );
    }
}
impl std::ops::Sub<DipoleAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        8       10        0
    fn sub(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
        );
    }
}
impl std::ops::Sub<DipoleAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        1        0        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        7        7        0
    fn sub(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            (self.group1().xyz() - other.group0().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
        );
    }
}
impl std::ops::Sub<DipoleAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        8        6        0
    fn sub(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0(),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::SubAssign<DipoleAtOrigin> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: DipoleAtOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0(),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<DipoleInversion> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        1        0        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        4        5        0
    //  no simd       12       11        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0(),
            // e23, e31, e12, e45
            (self.group1().xyz() - other.group1().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2() - other.group2(),
            // e4235, e4315, e4125, e3215
            other.group3().xyz().with_w(self[e3215] - other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<DipoleInversionAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        9       11        0
    fn sub(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2() - other.group1(),
            // e4235, e4315, e4125, e3215
            other.group2().xyz().with_w(self[e3215] - other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<DipoleInversionAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        1        0        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        8       11        0
    fn sub(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            (self.group1().xyz() - other.group0().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            other.group2().xyz().with_w(self[e3215] - other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<DipoleInversionAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        3        4        0
    //  no simd       12        4        0
    fn sub(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0(),
            // e23, e31, e12, e3215
            self.group1() + Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
            // e15, e25, e35, e1234
            self.group2() - other.group1(),
        );
    }
}
impl std::ops::SubAssign<DipoleInversionAtOrigin> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: DipoleInversionAtOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0(),
            // e23, e31, e12, e3215
            self.group1() + Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
            // e15, e25, e35, e1234
            self.group2() - other.group1(),
        );
    }
}
impl std::ops::Sub<DipoleInversionOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        8       12        0
    fn sub(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            other.group1().yzw().with_w(self[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<DipoleInversionOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        3        3        0
    //  no simd       12        3        0
    fn sub(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0(),
            // e23, e31, e12, e3215
            self.group1() - other.group1().with_w(other[e3215]),
            // e15, e25, e35, e1234
            self.group2() - other.group2(),
        );
    }
}
impl std::ops::SubAssign<DipoleInversionOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: DipoleInversionOrthogonalOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0(),
            // e23, e31, e12, e3215
            self.group1() - other.group1().with_w(other[e3215]),
            // e15, e25, e35, e1234
            self.group2() - other.group2(),
        );
    }
}
impl std::ops::Sub<DipoleOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        4        7        0
    fn sub(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
        );
    }
}
impl std::ops::Sub<DipoleOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        9        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        3        9        0
    //  no simd       12        9        0
    fn sub(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::SubAssign<DipoleOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: DipoleOrthogonalOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<DualNum> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        6        0
    fn sub(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<FlatOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
        );
    }
}
impl std::ops::Sub<FlatPoint> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        4        7        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
        );
    }
}
impl std::ops::Sub<FlatPointAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        4        3        0
    fn sub(self, other: FlatPointAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::SubAssign<FlatPointAtInfinity> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: FlatPointAtInfinity) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<Flector> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        5       11        0
    fn sub(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            other.group1().xyz().with_w(self[e3215] - other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<FlectorAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        4        0
    fn sub(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            self.group1() + Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::SubAssign<FlectorAtInfinity> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: FlectorAtInfinity) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            self.group1() + Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<FlectorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            other.group0().yzw().with_w(self[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<Horizon> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            self.group1() + Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<Horizon> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: Horizon) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            self.group1() + Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::Sub<Infinity> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Line> for VersorOddOrthogonalOrigin {
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
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<LineAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: LineAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<LineOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: LineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Motor> for VersorOddOrthogonalOrigin {
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
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MotorAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: MotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MotorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: MotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MultiVector> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd2        0        1        0
    //    simd3        3        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        6        8        0
    //  no simd       12       25        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            other.group1() * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            (self.group0().xyz() - other.group3().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            self.group2().xyz() - other.group4(),
            // e23, e31, e12
            self.group1().xyz() - other.group5(),
            // e415, e425, e435, e321
            other.group6() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group7() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group8() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] - other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<MysteryCircle> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: MysteryCircle) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MysteryCircleRotor> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        6        0
    fn sub(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MysteryDipole> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        3        4        0
    fn sub(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            (self.group1().xyz() - other.group0().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
        );
    }
}
impl std::ops::Sub<MysteryDipoleInversion> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        3        8        0
    fn sub(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            (self.group1().xyz() - other.group0().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            other.group1().with_w(self[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<MysteryVersorEven> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn sub(self, other: MysteryVersorEven) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            (other.group0().yzw() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MysteryVersorOdd> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        0        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        7        9        0
    fn sub(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
            // e23, e31, e12, e45
            (self.group1().xyz() - other.group1().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            other.group0().yzw().with_w(self[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<NullCircleAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: NullCircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<NullDipoleAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        4        3        0
    fn sub(self, other: NullDipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0(),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<NullDipoleAtOrigin> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: NullDipoleAtOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0(),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::Sub<NullDipoleInversionAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        4        0
    fn sub(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0(),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
        );
    }
}
impl std::ops::SubAssign<NullDipoleInversionAtOrigin> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: NullDipoleInversionAtOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0(),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
        );
    }
}
impl std::ops::Sub<NullSphereAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
        );
    }
}
impl std::ops::SubAssign<NullSphereAtOrigin> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: NullSphereAtOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
        );
    }
}
impl std::ops::Sub<NullVersorEvenAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Origin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Plane> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        1        4        0
    fn sub(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(self[e3215] - other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<PlaneOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            other.group0().with_w(self[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<RoundPoint> for VersorOddOrthogonalOrigin {
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
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<RoundPointAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Scalar> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<Scalar> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::Sub<Sphere> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        5        5        0
    fn sub(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(self[e3215] - other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<SphereAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        8        2        0
    fn sub(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            self.group1() + Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
        );
    }
}
impl std::ops::SubAssign<SphereAtOrigin> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: SphereAtOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            self.group1() + Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
        );
    }
}
impl std::ops::Sub<SphereOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        4        5        0
    fn sub(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(self[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<VersorEven> for VersorOddOrthogonalOrigin {
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
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       16        0
    fn sub(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       13        0
    fn sub(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            (other.group0().yzw() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       11        0
    fn sub(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn sub(self, other: VersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       15        0
    fn sub(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            other.group2() * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            self.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorOdd> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        4        2        0
    //  no simd       12        8        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() - other.group0(),
            // e23, e31, e12, e45
            (self.group1().xyz() - other.group1().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2() - other.group2(),
            // e4235, e4315, e4125, e3215
            other.group3().xyz().with_w(self[e3215] - other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<VersorOddAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        4        0
    //    simd3        1        0        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        4        6        0
    //  no simd       12       12        0
    fn sub(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
            // e23, e31, e12, e45
            (self.group1().xyz() - other.group1().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            other.group2().xyz().with_w(self[e3215] - other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<VersorOddOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        3        0        0
    // no simd       12        0        0
    fn sub(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() - other.group0(),
            // e23, e31, e12, e3215
            self.group1() - other.group1(),
            // e15, e25, e35, e1234
            self.group2() - other.group2(),
        );
    }
}
impl std::ops::SubAssign<VersorOddOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: VersorOddOrthogonalOrigin) {
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() - other.group0(),
            // e23, e31, e12, e3215
            self.group1() - other.group1(),
            // e15, e25, e35, e1234
            self.group2() - other.group2(),
        );
    }
}

impl TryFrom<AntiCircleRotor> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(anti_circle_rotor: AntiCircleRotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotor do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            anti_circle_rotor.group0().with_w(anti_circle_rotor[scalar]),
            // e23, e31, e12, e3215
            anti_circle_rotor.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            anti_circle_rotor.group2().xyz().with_w(0.0),
        ));
    }
}

impl TryFrom<AntiCircleRotorAtInfinity> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(anti_circle_rotor_at_infinity: AntiCircleRotorAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotorAtInfinity do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(anti_circle_rotor_at_infinity[scalar]),
            // e23, e31, e12, e3215
            anti_circle_rotor_at_infinity.group0().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            anti_circle_rotor_at_infinity.group1().xyz().with_w(0.0),
        ));
    }
}

impl TryFrom<AntiMysteryCircleRotor> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(anti_mystery_circle_rotor: AntiMysteryCircleRotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_mystery_circle_rotor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMysteryCircleRotor do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(anti_mystery_circle_rotor[scalar]),
            // e23, e31, e12, e3215
            anti_mystery_circle_rotor.group0().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<Dipole> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(dipole: Dipole) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Dipole do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            dipole.group0().with_w(0.0),
            // e23, e31, e12, e3215
            dipole.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            dipole.group2().with_w(0.0),
        ));
    }
}

impl TryFrom<DipoleAligningOrigin> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(dipole_aligning_origin: DipoleAligningOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_aligning_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleAligningOrigin do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            dipole_aligning_origin.group0().xyz().with_w(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            dipole_aligning_origin.group1().with_w(0.0),
        ));
    }
}

impl TryFrom<DipoleAtInfinity> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(dipole_at_infinity: DipoleAtInfinity) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleAtInfinity do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            dipole_at_infinity.group0().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            dipole_at_infinity.group1().with_w(0.0),
        ));
    }
}

impl TryFrom<DipoleInversion> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(dipole_inversion: DipoleInversion) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
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
            let mut error = "Elements from DipoleInversion do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            dipole_inversion.group0().with_w(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from([dipole_inversion[e23], dipole_inversion[e31], dipole_inversion[e12], dipole_inversion[e3215]]),
            // e15, e25, e35, e1234
            dipole_inversion.group2(),
        ));
    }
}

impl TryFrom<DipoleInversionAligningOrigin> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(dipole_inversion_aligning_origin: DipoleInversionAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_aligning_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_aligning_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_aligning_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_aligning_origin[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversionAligningOrigin do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            dipole_inversion_aligning_origin.group0().xyz().with_w(0.0),
            // e23, e31, e12, e3215
            Simd32x3::from(0.0).with_w(dipole_inversion_aligning_origin[e3215]),
            // e15, e25, e35, e1234
            dipole_inversion_aligning_origin.group1(),
        ));
    }
}

impl TryFrom<DipoleInversionAtInfinity> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(dipole_inversion_at_infinity: DipoleInversionAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_infinity[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_infinity[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_infinity[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversionAtInfinity do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from([
                dipole_inversion_at_infinity[e23],
                dipole_inversion_at_infinity[e31],
                dipole_inversion_at_infinity[e12],
                dipole_inversion_at_infinity[e3215],
            ]),
            // e15, e25, e35, e1234
            dipole_inversion_at_infinity.group1().with_w(0.0),
        ));
    }
}

impl TryFrom<DipoleInversionOnOrigin> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(dipole_inversion_on_origin: DipoleInversionOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_on_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_on_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_on_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversionOnOrigin do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            dipole_inversion_on_origin.group0().xyz().with_w(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(dipole_inversion_on_origin[e1234]),
        ));
    }
}

impl TryFrom<DipoleOnOrigin> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(dipole_on_origin: DipoleOnOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleOnOrigin do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            dipole_on_origin.group0().xyz().with_w(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<FlatPoint> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(flat_point: FlatPoint) -> Result<Self, Self::Error> {
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
            let mut error = "Elements from FlatPoint do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            flat_point.group0().xyz().with_w(0.0),
        ));
    }
}

impl TryFrom<Flector> for VersorOddOrthogonalOrigin {
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
            let mut error = "Elements from Flector do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x3::from(0.0).with_w(flector[e3215]),
            // e15, e25, e35, e1234
            flector.group0().xyz().with_w(0.0),
        ));
    }
}

impl TryFrom<MultiVector> for VersorOddOrthogonalOrigin {
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
        if fail {
            let mut error = "Elements from MultiVector do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([multi_vector[e41], multi_vector[e42], multi_vector[e43], multi_vector[scalar]]),
            // e23, e31, e12, e3215
            multi_vector.group5().with_w(multi_vector[e3215]),
            // e15, e25, e35, e1234
            multi_vector.group4().with_w(multi_vector[e1234]),
        ));
    }
}

impl TryFrom<MysteryDipole> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(mystery_dipole: MysteryDipole) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_dipole[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryDipole do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            mystery_dipole.group0().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<MysteryDipoleInversion> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(mystery_dipole_inversion: MysteryDipoleInversion) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_dipole_inversion[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_dipole_inversion[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_dipole_inversion[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_dipole_inversion[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryDipoleInversion do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            mystery_dipole_inversion.group0().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<MysteryVersorOdd> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(mystery_versor_odd: MysteryVersorOdd) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_versor_odd[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_odd[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_odd[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_odd[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryVersorOdd do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(mystery_versor_odd[scalar]),
            // e23, e31, e12, e3215
            mystery_versor_odd.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<Plane> for VersorOddOrthogonalOrigin {
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
            let mut error = "Elements from Plane do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x3::from(0.0).with_w(plane[e3215]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<Sphere> for VersorOddOrthogonalOrigin {
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
        if fail {
            let mut error = "Elements from Sphere do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x3::from(0.0).with_w(sphere[e3215]),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(sphere[e1234]),
        ));
    }
}

impl TryFrom<SphereOnOrigin> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(sphere_on_origin: SphereOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = sphere_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = sphere_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = sphere_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from SphereOnOrigin do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(sphere_on_origin[e1234]),
        ));
    }
}

impl TryFrom<VersorOdd> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(versor_odd: VersorOdd) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
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
            let mut error = "Elements from VersorOdd do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            versor_odd.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([versor_odd[e23], versor_odd[e31], versor_odd[e12], versor_odd[e3215]]),
            // e15, e25, e35, e1234
            versor_odd.group2(),
        ));
    }
}

impl TryFrom<VersorOddAtInfinity> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(versor_odd_at_infinity: VersorOddAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd_at_infinity[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOddAtInfinity do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(versor_odd_at_infinity[scalar]),
            // e23, e31, e12, e3215
            Simd32x4::from([versor_odd_at_infinity[e23], versor_odd_at_infinity[e31], versor_odd_at_infinity[e12], versor_odd_at_infinity[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([versor_odd_at_infinity[e15], versor_odd_at_infinity[e25], versor_odd_at_infinity[e35], 0.0]),
        ));
    }
}
