use crate::traits::GeometricProduct;
use crate::traits::RightDual;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 325
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         1       1       0
//  Average:         8      13       0
//  Maximum:       126     158       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       4       0
//  Average:        17      21       0
//  Maximum:       288     320       0
impl std::ops::Add<AntiCircleOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: AntiCircleOnOrigin) -> Self::Output {
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            other.group0() + self.group0(),
            // e23, e31, e12
            other.group1() + self.group1(),
            // e15, e25, e35, scalar
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<AntiCircleOnOrigin> for AntiCircleRotorAligningOrigin {
    fn add_assign(&mut self, other: AntiCircleOnOrigin) {
        *self = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            other.group0() + self.group0(),
            // e23, e31, e12
            other.group1() + self.group1(),
            // e15, e25, e35, scalar
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiCircleRotor> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0() + self.group0(),
            // e23, e31, e12, e45
            (self.group1() + other.group1().xyz()).with_w(other[e45]),
            // e15, e25, e35, scalar
            other.group2() + self.group2(),
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOrigin> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn add(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            other.group0() + self.group0(),
            // e23, e31, e12
            other.group1() + self.group1(),
            // e15, e25, e35, scalar
            other.group2() + self.group2(),
        );
    }
}
impl std::ops::AddAssign<AntiCircleRotorAligningOrigin> for AntiCircleRotorAligningOrigin {
    fn add_assign(&mut self, other: AntiCircleRotorAligningOrigin) {
        *self = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            other.group0() + self.group0(),
            // e23, e31, e12
            other.group1() + self.group1(),
            // e15, e25, e35, scalar
            other.group2() + self.group2(),
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOriginAtInfinity> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1() + other.group0(),
            // e15, e25, e35, scalar
            self.group2() + other.group1(),
        );
    }
}
impl std::ops::AddAssign<AntiCircleRotorAligningOriginAtInfinity> for AntiCircleRotorAligningOrigin {
    fn add_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        *self = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1() + other.group0(),
            // e15, e25, e35, scalar
            self.group2() + other.group1(),
        );
    }
}
impl std::ops::Add<AntiCircleRotorAtInfinity> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            (self.group1() + other.group0().xyz()).with_w(other[e45]),
            // e15, e25, e35, scalar
            self.group2() + other.group1(),
        );
    }
}
impl std::ops::Add<AntiCircleRotorOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn add(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0() + other.group0().xyz(),
            // e23, e31, e12
            self.group1() + other.group1(),
            // e15, e25, e35, scalar
            self.group2() + Simd32x3::from(0.0).with_w(other[scalar]),
        );
    }
}
impl std::ops::AddAssign<AntiCircleRotorOnOrigin> for AntiCircleRotorAligningOrigin {
    fn add_assign(&mut self, other: AntiCircleRotorOnOrigin) {
        use crate::elements::*;
        *self = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0() + other.group0().xyz(),
            // e23, e31, e12
            self.group1() + other.group1(),
            // e15, e25, e35, scalar
            self.group2() + Simd32x3::from(0.0).with_w(other[scalar]),
        );
    }
}
impl std::ops::Add<AntiDipoleInversion> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiDipoleInversionAtInfinity> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOnOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOrthogonalOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group1().with_w(0.0),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiDipoleOnOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiDualNum> for AntiCircleRotorAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar] + other[scalar]),
            // e23, e31, e12, e3215
            self.group1().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(other[e1234]),
        );
    }
}
impl std::ops::Add<AntiFlatOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiFlatPoint> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiFlector> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiFlectorOnOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiLine> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1() + other.group0(),
            // e15, e25, e35, scalar
            (other.group1() + self.group2().xyz()).with_w(self[scalar]),
        );
    }
}
impl std::ops::AddAssign<AntiLine> for AntiCircleRotorAligningOrigin {
    fn add_assign(&mut self, other: AntiLine) {
        use crate::elements::*;
        *self = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1() + other.group0(),
            // e15, e25, e35, scalar
            (other.group1() + self.group2().xyz()).with_w(self[scalar]),
        );
    }
}
impl std::ops::Add<AntiLineOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: AntiLineOnOrigin) -> Self::Output {
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1() + other.group0(),
            // e15, e25, e35, scalar
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<AntiLineOnOrigin> for AntiCircleRotorAligningOrigin {
    fn add_assign(&mut self, other: AntiLineOnOrigin) {
        *self = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1() + other.group0(),
            // e15, e25, e35, scalar
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiMotor> for AntiCircleRotorAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar] + other[scalar]),
            // e23, e31, e12, e3215
            (self.group1() + other.group0().xyz()).with_w(other[e3215]),
            // e15, e25, e35, e1234
            (self.group2().xyz() + other.group1().xyz()).with_w(0.0),
        );
    }
}
impl std::ops::Add<AntiMotorOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1() + other.group0().xyz(),
            // e15, e25, e35, scalar
            self.group2() + Simd32x3::from(0.0).with_w(other[scalar]),
        );
    }
}
impl std::ops::AddAssign<AntiMotorOnOrigin> for AntiCircleRotorAligningOrigin {
    fn add_assign(&mut self, other: AntiMotorOnOrigin) {
        use crate::elements::*;
        *self = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1() + other.group0().xyz(),
            // e15, e25, e35, scalar
            self.group2() + Simd32x3::from(0.0).with_w(other[scalar]),
        );
    }
}
impl std::ops::Add<AntiMysteryCircleRotor> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            (self.group1() + other.group0().xyz()).with_w(other[e45]),
            // e15, e25, e35, scalar
            self.group2() + Simd32x3::from(0.0).with_w(other[scalar]),
        );
    }
}
impl std::ops::Add<AntiMysteryDipoleInversion> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiPlane> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiPlaneOnOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiScalar> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiSphereOnOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiVersorEvenOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            other.group0() + self.group0().with_w(self[scalar]),
            // e23, e31, e12, e3215
            (self.group1() + other.group1().xyz()).with_w(0.0),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(other[e1234]),
        );
    }
}
impl std::ops::Add<Circle> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<CircleAligningOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group1().with_w(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<CircleAtInfinity> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<CircleAtOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<CircleOnOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group1().with_w(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<CircleOrthogonalOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<CircleRotor> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<CircleRotorAligningOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group1().with_w(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<CircleRotorAligningOriginAtInfinity> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group0().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<CircleRotorAtInfinity> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<CircleRotorOnOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group1().with_w(0.0),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Dipole> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        3        0        0
    // no simd        9        0        0
    fn add(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0() + other.group0(),
            // e23, e31, e12, e45
            (self.group1() + other.group1().xyz()).with_w(other[e45]),
            // e15, e25, e35, scalar
            (other.group2() + self.group2().xyz()).with_w(self[scalar]),
        );
    }
}
impl std::ops::Add<DipoleAligningOrigin> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0() + other.group0().xyz(),
            // e23, e31, e12, e45
            self.group1().with_w(other[e45]),
            // e15, e25, e35, scalar
            (other.group1() + self.group2().xyz()).with_w(self[scalar]),
        );
    }
}
impl std::ops::Add<DipoleAtInfinity> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            (self.group1() + other.group0().xyz()).with_w(other[e45]),
            // e15, e25, e35, scalar
            (other.group1() + self.group2().xyz()).with_w(self[scalar]),
        );
    }
}
impl std::ops::Add<DipoleAtOrigin> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0() + other.group0(),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, scalar
            (other.group1() + self.group2().xyz()).with_w(self[scalar]),
        );
    }
}
impl std::ops::AddAssign<DipoleAtOrigin> for AntiCircleRotorAligningOrigin {
    fn add_assign(&mut self, other: DipoleAtOrigin) {
        use crate::elements::*;
        *self = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0() + other.group0(),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, scalar
            (other.group1() + self.group2().xyz()).with_w(self[scalar]),
        );
    }
}
impl std::ops::Add<DipoleInversion> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        3        0        0
    // no simd        9        0        0
    fn add(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0() + other.group0()).with_w(self[scalar]),
            // e23, e31, e12, e45
            (self.group1() + other.group1().xyz()).with_w(other[e45]),
            // e15, e25, e35, e1234
            (self.group2().xyz() + other.group2().xyz()).with_w(other[e1234]),
            // e4235, e4315, e4125, e3215
            other.group3(),
        );
    }
}
impl std::ops::Add<DipoleInversionAligningOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0() + other.group0().xyz()).with_w(self[scalar]),
            // e23, e31, e12, e45
            self.group1().with_w(other[e45]),
            // e15, e25, e35, e1234
            (self.group2().xyz() + other.group1().xyz()).with_w(other[e1234]),
            // e4235, e4315, e4125, e3215
            other.group2(),
        );
    }
}
impl std::ops::Add<DipoleInversionAtInfinity> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar]),
            // e23, e31, e12, e45
            (self.group1() + other.group0().xyz()).with_w(other[e45]),
            // e15, e25, e35, e1234
            (other.group1() + self.group2().xyz()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            other.group2(),
        );
    }
}
impl std::ops::Add<DipoleInversionAtOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            (self.group0() + other.group0().xyz()).with_w(self[scalar]),
            // e23, e31, e12, e3215
            self.group1().with_w(other[e3215]),
            // e15, e25, e35, e1234
            (self.group2().xyz() + other.group1().xyz()).with_w(other[e1234]),
        );
    }
}
impl std::ops::Add<DipoleInversionOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0() + other.group0().xyz()).with_w(self[scalar]),
            // e23, e31, e12, e45
            self.group1().with_w(other[e45]),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], 0.0]),
        );
    }
}
impl std::ops::Add<DipoleInversionOrthogonalOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        3        0        0
    // no simd        9        0        0
    fn add(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            (self.group0() + other.group0().xyz()).with_w(self[scalar]),
            // e23, e31, e12, e3215
            (self.group1() + other.group1()).with_w(other[e3215]),
            // e15, e25, e35, e1234
            (self.group2().xyz() + other.group2().xyz()).with_w(other[e1234]),
        );
    }
}
impl std::ops::Add<DipoleOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0() + other.group0().xyz(),
            // e23, e31, e12, e45
            self.group1().with_w(other[e45]),
            // e15, e25, e35, scalar
            self.group2(),
        );
    }
}
impl std::ops::Add<DipoleOrthogonalOrigin> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        3        0        0
    // no simd        9        0        0
    fn add(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0() + other.group0(),
            // e23, e31, e12
            self.group1() + other.group1(),
            // e15, e25, e35, scalar
            (other.group2() + self.group2().xyz()).with_w(self[scalar]),
        );
    }
}
impl std::ops::AddAssign<DipoleOrthogonalOrigin> for AntiCircleRotorAligningOrigin {
    fn add_assign(&mut self, other: DipoleOrthogonalOrigin) {
        use crate::elements::*;
        *self = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0() + other.group0(),
            // e23, e31, e12
            self.group1() + other.group1(),
            // e15, e25, e35, scalar
            (other.group2() + self.group2().xyz()).with_w(self[scalar]),
        );
    }
}
impl std::ops::Add<DualNum> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<FlatOrigin> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotor;
    fn add(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1().with_w(other[e45]),
            // e15, e25, e35, scalar
            self.group2(),
        );
    }
}
impl std::ops::Add<FlatPoint> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1().with_w(other[e45]),
            // e15, e25, e35, scalar
            self.group2() + other.group0().xyz().with_w(0.0),
        );
    }
}
impl std::ops::Add<FlatPointAtInfinity> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: FlatPointAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, scalar
            (other.group0() + self.group2().xyz()).with_w(self[scalar]),
        );
    }
}
impl std::ops::AddAssign<FlatPointAtInfinity> for AntiCircleRotorAligningOrigin {
    fn add_assign(&mut self, other: FlatPointAtInfinity) {
        use crate::elements::*;
        *self = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, scalar
            (other.group0() + self.group2().xyz()).with_w(self[scalar]),
        );
    }
}
impl std::ops::Add<Flector> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar]),
            // e23, e31, e12, e45
            self.group1().with_w(other[e45]),
            // e15, e25, e35, e1234
            (self.group2().xyz() + other.group0().xyz()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            other.group1(),
        );
    }
}
impl std::ops::Add<FlectorAtInfinity> for AntiCircleRotorAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar]),
            // e23, e31, e12, e3215
            self.group1().with_w(other[e3215]),
            // e15, e25, e35, e1234
            (self.group2().xyz() + other.group0().xyz()).with_w(0.0),
        );
    }
}
impl std::ops::Add<FlectorOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    fn add(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar]),
            // e23, e31, e12, e45
            self.group1().with_w(other[e45]),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], 0.0]),
        );
    }
}
impl std::ops::Add<Horizon> for AntiCircleRotorAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar]),
            // e23, e31, e12, e3215
            self.group1().with_w(other[e3215]),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(0.0),
        );
    }
}
impl std::ops::Add<Infinity> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Line> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group0().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<LineAtInfinity> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<LineOnOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group0().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Motor> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MotorAtInfinity> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MotorOnOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MultiVector> for AntiCircleRotorAligningOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        1        0        0
    //    simd3        3        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       11        0        0
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
            (self.group0() + other.group3().xyz()).with_w(other[e45]),
            // e15, e25, e35
            other.group4() + self.group2().xyz(),
            // e23, e31, e12
            self.group1() + other.group5(),
            // e415, e425, e435, e321
            other.group6(),
            // e423, e431, e412
            other.group7(),
            // e235, e315, e125
            other.group8(),
            // e1234, e4235, e4315, e4125
            other.group9(),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<MysteryCircle> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MysteryCircleRotor> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MysteryDipole> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            (self.group1() + other.group0().xyz()).with_w(other[e45]),
            // e15, e25, e35, scalar
            self.group2(),
        );
    }
}
impl std::ops::Add<MysteryDipoleInversion> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar]),
            // e23, e31, e12, e45
            (self.group1() + other.group0().xyz()).with_w(other[e45]),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            other.group1().with_w(0.0),
        );
    }
}
impl std::ops::Add<MysteryVersorEven> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MysteryVersorOdd> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar] + other[scalar]),
            // e23, e31, e12, e45
            (self.group1() + other.group1().xyz()).with_w(other[e45]),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], 0.0]),
        );
    }
}
impl std::ops::Add<NullCircleAtOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullDipoleAtOrigin> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: NullDipoleAtOrigin) -> Self::Output {
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0() + other.group0(),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, scalar
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<NullDipoleAtOrigin> for AntiCircleRotorAligningOrigin {
    fn add_assign(&mut self, other: NullDipoleAtOrigin) {
        *self = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0() + other.group0(),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, scalar
            self.group2(),
        );
    }
}
impl std::ops::Add<NullDipoleInversionAtOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            (self.group0() + other.group0().xyz()).with_w(self[scalar]),
            // e23, e31, e12, e3215
            self.group1().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(other[e1234]),
        );
    }
}
impl std::ops::Add<NullSphereAtOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    fn add(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar]),
            // e23, e31, e12, e3215
            self.group1().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(other[e1234]),
        );
    }
}
impl std::ops::Add<NullVersorEvenAtOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Origin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Plane> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    fn add(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar]),
            // e23, e31, e12, e45
            self.group1().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            other.group0(),
        );
    }
}
impl std::ops::Add<PlaneOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    fn add(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar]),
            // e23, e31, e12, e45
            self.group1().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            other.group0().with_w(0.0),
        );
    }
}
impl std::ops::Add<RoundPoint> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<RoundPointAtOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Scalar> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, scalar
            self.group2() + Simd32x3::from(0.0).with_w(other[scalar]),
        );
    }
}
impl std::ops::AddAssign<Scalar> for AntiCircleRotorAligningOrigin {
    fn add_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, scalar
            self.group2() + Simd32x3::from(0.0).with_w(other[scalar]),
        );
    }
}
impl std::ops::Add<Sphere> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar]),
            // e23, e31, e12, e45
            self.group1().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(other[e1234]),
            // e4235, e4315, e4125, e3215
            other.group0(),
        );
    }
}
impl std::ops::Add<SphereAtOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    fn add(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar]),
            // e23, e31, e12, e3215
            self.group1().with_w(other[e3215]),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(other[e1234]),
        );
    }
}
impl std::ops::Add<SphereOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    fn add(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar]),
            // e23, e31, e12, e45
            self.group1().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(other[e1234]),
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(0.0),
        );
    }
}
impl std::ops::Add<VersorEven> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<VersorEvenAligningOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group1().xyz().with_w(0.0),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<VersorEvenAtInfinity> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<VersorEvenAtOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<VersorEvenOnOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group1().xyz().with_w(0.0),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<VersorEvenOrthogonalOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<VersorOdd> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn add(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() + self.group0().with_w(self[scalar]),
            // e23, e31, e12, e45
            (self.group1() + other.group1().xyz()).with_w(other[e45]),
            // e15, e25, e35, e1234
            (self.group2().xyz() + other.group2().xyz()).with_w(other[e1234]),
            // e4235, e4315, e4125, e3215
            other.group3(),
        );
    }
}
impl std::ops::Add<VersorOddAtInfinity> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        7        0        0
    fn add(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar] + other[scalar]),
            // e23, e31, e12, e45
            (self.group1() + other.group1().xyz()).with_w(other[e45]),
            // e15, e25, e35, e1234
            (self.group2().xyz() + other.group0().yzw()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            other.group2(),
        );
    }
}
impl std::ops::Add<VersorOddOrthogonalOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn add(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            other.group0() + self.group0().with_w(self[scalar]),
            // e23, e31, e12, e3215
            (self.group1() + other.group1().xyz()).with_w(other[e3215]),
            // e15, e25, e35, e1234
            (self.group2().xyz() + other.group2().xyz()).with_w(other[e1234]),
        );
    }
}

impl From<AntiCircleOnOrigin> for AntiCircleRotorAligningOrigin {
    fn from(from_anti_circle_on_origin: AntiCircleOnOrigin) -> Self {
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            from_anti_circle_on_origin.group0(),
            // e23, e31, e12
            from_anti_circle_on_origin.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiCircleRotorAligningOriginAtInfinity> for AntiCircleRotorAligningOrigin {
    fn from(from_anti_circle_rotor_aligning_origin_at_infinity: AntiCircleRotorAligningOriginAtInfinity) -> Self {
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            from_anti_circle_rotor_aligning_origin_at_infinity.group0(),
            // e15, e25, e35, scalar
            from_anti_circle_rotor_aligning_origin_at_infinity.group1(),
        );
    }
}

impl From<AntiCircleRotorOnOrigin> for AntiCircleRotorAligningOrigin {
    fn from(from_anti_circle_rotor_on_origin: AntiCircleRotorOnOrigin) -> Self {
        use crate::elements::*;
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            from_anti_circle_rotor_on_origin.group0().xyz(),
            // e23, e31, e12
            from_anti_circle_rotor_on_origin.group1(),
            // e15, e25, e35, scalar
            Simd32x3::from(0.0).with_w(from_anti_circle_rotor_on_origin[scalar]),
        );
    }
}

impl From<AntiLine> for AntiCircleRotorAligningOrigin {
    fn from(from_anti_line: AntiLine) -> Self {
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            from_anti_line.group0(),
            // e15, e25, e35, scalar
            from_anti_line.group1().with_w(0.0),
        );
    }
}

impl From<AntiLineOnOrigin> for AntiCircleRotorAligningOrigin {
    fn from(from_anti_line_on_origin: AntiLineOnOrigin) -> Self {
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            from_anti_line_on_origin.group0(),
            // e15, e25, e35, scalar
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiMotorOnOrigin> for AntiCircleRotorAligningOrigin {
    fn from(from_anti_motor_on_origin: AntiMotorOnOrigin) -> Self {
        use crate::elements::*;
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            from_anti_motor_on_origin.group0().xyz(),
            // e15, e25, e35, scalar
            Simd32x3::from(0.0).with_w(from_anti_motor_on_origin[scalar]),
        );
    }
}

impl From<DipoleAtOrigin> for AntiCircleRotorAligningOrigin {
    fn from(from_dipole_at_origin: DipoleAtOrigin) -> Self {
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            from_dipole_at_origin.group0(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, scalar
            from_dipole_at_origin.group1().with_w(0.0),
        );
    }
}

impl From<DipoleOrthogonalOrigin> for AntiCircleRotorAligningOrigin {
    fn from(from_dipole_orthogonal_origin: DipoleOrthogonalOrigin) -> Self {
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            from_dipole_orthogonal_origin.group0(),
            // e23, e31, e12
            from_dipole_orthogonal_origin.group1(),
            // e15, e25, e35, scalar
            from_dipole_orthogonal_origin.group2().with_w(0.0),
        );
    }
}

impl From<FlatPointAtInfinity> for AntiCircleRotorAligningOrigin {
    fn from(from_flat_point_at_infinity: FlatPointAtInfinity) -> Self {
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, scalar
            from_flat_point_at_infinity.group0().with_w(0.0),
        );
    }
}

impl From<NullDipoleAtOrigin> for AntiCircleRotorAligningOrigin {
    fn from(from_null_dipole_at_origin: NullDipoleAtOrigin) -> Self {
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            from_null_dipole_at_origin.group0(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, scalar
            Simd32x4::from(0.0),
        );
    }
}

impl From<Scalar> for AntiCircleRotorAligningOrigin {
    fn from(from_scalar: Scalar) -> Self {
        use crate::elements::*;
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, scalar
            Simd32x3::from(0.0).with_w(from_scalar[scalar]),
        );
    }
}
impl std::ops::Mul<AntiCircleOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       41        0
    //    simd3        0        4        0
    //    simd4        6        2        0
    // Totals...
    // yes simd       26       47        0
    //  no simd       44       61        0
    fn mul(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       59        0
    //    simd3        0        9        0
    //    simd4       15        6        0
    // Totals...
    // yes simd       50       74        0
    //  no simd       95      110        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       56        0
    //    simd3        0        8        0
    //    simd4       13        5        0
    // Totals...
    // yes simd       46       69        0
    //  no simd       85      100        0
    fn mul(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOriginAtInfinity> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       42        0
    //    simd3        0        3        0
    //    simd4        8        5        0
    // Totals...
    // yes simd       31       50        0
    //  no simd       55       71        0
    fn mul(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAtInfinity> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       43        0
    //    simd3        0        3        0
    //    simd4       10        7        0
    // Totals...
    // yes simd       36       53        0
    //  no simd       66       80        0
    fn mul(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       48        0
    //    simd3        0        5        0
    //    simd4        7        2        0
    // Totals...
    // yes simd       33       55        0
    //  no simd       54       71        0
    fn mul(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       42       69        0
    //    simd3        0       11        0
    //    simd4       23       12        0
    // Totals...
    // yes simd       65       92        0
    //  no simd      134      150        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionAtInfinity> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       58        0
    //    simd3        0        7        0
    //    simd4       15        8        0
    // Totals...
    // yes simd       49       73        0
    //  no simd       94      111        0
    fn mul(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       27        0
    //    simd3        0        3        0
    //    simd4       13       11        0
    // Totals...
    // yes simd       29       41        0
    //  no simd       68       80        0
    fn mul(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOrthogonalOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       57        0
    //    simd3        0        7        0
    //    simd4       15        8        0
    // Totals...
    // yes simd       50       72        0
    //  no simd       95      110        0
    fn mul(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       25        0
    //    simd3        0        1        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       13       30        0
    //  no simd       25       44        0
    fn mul(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        2        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        6       20        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       14        0
    fn mul(self, other: AntiFlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       16        0
    //    simd3        0        5        0
    //    simd4        5        3        0
    // Totals...
    // yes simd       15       24        0
    //  no simd       30       43        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       42        0
    //    simd3        0        1        0
    //    simd4       10        9        0
    // Totals...
    // yes simd       35       52        0
    //  no simd       65       81        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlectorOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       16        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       11       23        0
    //  no simd       32       44        0
    fn mul(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       40        0
    //    simd3        0        7        0
    //    simd4        7        0        0
    // Totals...
    // yes simd       24       47        0
    //  no simd       45       61        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLineOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       21        0
    //    simd3        0        3        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        9       24        0
    //  no simd       18       30        0
    fn mul(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       44        0
    //    simd3        0        3        0
    //    simd4       10        7        0
    // Totals...
    // yes simd       38       54        0
    //  no simd       68       81        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotorOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       16       28        0
    //  no simd       28       40        0
    fn mul(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryCircleRotor> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       21        0
    //    simd3        3        6        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       16       30        0
    //  no simd       34       51        0
    fn mul(self, other: AntiMysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryDipoleInversion> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       30        0
    //    simd3        0        4        0
    //    simd4       11        7        0
    // Totals...
    // yes simd       25       41        0
    //  no simd       58       70        0
    fn mul(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for AntiCircleRotorAligningOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       18        0
    //    simd3        1        3        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       11       25        0
    //  no simd       25       43        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlaneOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       20        0
    //    simd3        0        3        0
    //    simd4        4        1        0
    // Totals...
    // yes simd        6       24        0
    //  no simd       18       33        0
    fn mul(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for AntiCircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiSphereOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       20        0
    //    simd3        2        4        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       12       27        0
    //  no simd       25       44        0
    fn mul(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorEvenOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       51        0
    //    simd3        0        2        0
    //    simd4        8        6        0
    // Totals...
    // yes simd       40       59        0
    //  no simd       64       81        0
    fn mul(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       56        0
    //    simd3        0       12        0
    //    simd4       13        2        0
    // Totals...
    // yes simd       45       70        0
    //  no simd       84      100        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAligningOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       53        0
    //    simd3        0       11        0
    //    simd4       12        1        0
    // Totals...
    // yes simd       39       65        0
    //  no simd       75       90        0
    fn mul(self, other: CircleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtInfinity> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       47        0
    //    simd3        0        8        0
    //    simd4        8        0        0
    // Totals...
    // yes simd       30       55        0
    //  no simd       54       71        0
    fn mul(self, other: CircleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       36        0
    //    simd3        0        7        0
    //    simd4        8        1        0
    // Totals...
    // yes simd       23       44        0
    //  no simd       47       61        0
    fn mul(self, other: CircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       36        0
    //    simd3        0        4        0
    //    simd4        7        3        0
    // Totals...
    // yes simd       24       43        0
    //  no simd       45       60        0
    fn mul(self, other: CircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOrthogonalOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       42        0
    //    simd3        0        7        0
    //    simd4        9        2        0
    // Totals...
    // yes simd       30       51        0
    //  no simd       57       71        0
    fn mul(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       60        0
    //    simd3        0       10        0
    //    simd4       15        5        0
    // Totals...
    // yes simd       50       75        0
    //  no simd       95      110        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       56        0
    //    simd3        0        8        0
    //    simd4       13        5        0
    // Totals...
    // yes simd       46       69        0
    //  no simd       85      100        0
    fn mul(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOriginAtInfinity> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       47        0
    //    simd3        0        4        0
    //    simd4        7        3        0
    // Totals...
    // yes simd       33       54        0
    //  no simd       54       71        0
    fn mul(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAtInfinity> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       51        0
    //    simd3        0        6        0
    //    simd4        9        3        0
    // Totals...
    // yes simd       37       60        0
    //  no simd       64       81        0
    fn mul(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       42        0
    //    simd3        0        4        0
    //    simd4        8        4        0
    // Totals...
    // yes simd       31       50        0
    //  no simd       55       70        0
    fn mul(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       54        0
    //    simd3        0       10        0
    //    simd4       13        4        0
    // Totals...
    // yes simd       45       68        0
    //  no simd       84      100        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAligningOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       43        0
    //    simd3        0        7        0
    //    simd4        9        2        0
    // Totals...
    // yes simd       30       52        0
    //  no simd       57       72        0
    fn mul(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtInfinity> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       44        0
    //    simd3        0        6        0
    //    simd4        8        2        0
    // Totals...
    // yes simd       31       52        0
    //  no simd       55       70        0
    fn mul(self, other: DipoleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       36        0
    //    simd3        0        7        0
    //    simd4        8        1        0
    // Totals...
    // yes simd       23       44        0
    //  no simd       47       61        0
    fn mul(self, other: DipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       42       71        0
    //    simd3        0       13        0
    //    simd4       23       10        0
    // Totals...
    // yes simd       65       94        0
    //  no simd      134      150        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAligningOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       51        0
    //    simd3        0        7        0
    //    simd4       19       12        0
    // Totals...
    // yes simd       51       70        0
    //  no simd      108      120        0
    fn mul(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtInfinity> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       43        0
    //    simd3        0        8        0
    //    simd4       19       11        0
    // Totals...
    // yes simd       43       62        0
    //  no simd      100      111        0
    fn mul(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       38        0
    //    simd3        0        5        0
    //    simd4       12        7        0
    // Totals...
    // yes simd       29       50        0
    //  no simd       65       81        0
    fn mul(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       43        0
    //    simd3        0        2        0
    //    simd4       10        8        0
    // Totals...
    // yes simd       38       53        0
    //  no simd       68       81        0
    fn mul(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOrthogonalOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       59        0
    //    simd3        0        9        0
    //    simd4       15        6        0
    // Totals...
    // yes simd       50       74        0
    //  no simd       95      110        0
    fn mul(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       16        0
    //    simd3        0        5        0
    //    simd4        5        3        0
    // Totals...
    // yes simd       15       24        0
    //  no simd       30       43        0
    fn mul(self, other: DipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOrthogonalOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       53        0
    //    simd3        0       11        0
    //    simd4       12        1        0
    // Totals...
    // yes simd       39       65        0
    //  no simd       75       90        0
    fn mul(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        2        5        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        8        0
    //  no simd        6       24        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatOrigin> for AntiCircleRotorAligningOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       13        0
    fn mul(self, other: FlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       20        0
    //    simd3        0        5        0
    //    simd4        5        2        0
    // Totals...
    // yes simd       13       27        0
    //  no simd       28       43        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPointAtInfinity> for AntiCircleRotorAligningOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       23        0
    //    simd3        0        2        0
    //    simd4        3        1        0
    // Totals...
    // yes simd        9       26        0
    //  no simd       18       33        0
    fn mul(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       29        0
    //    simd3        0        5        0
    //    simd4       13        9        0
    // Totals...
    // yes simd       29       43        0
    //  no simd       68       80        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorAtInfinity> for AntiCircleRotorAligningOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       20        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       13       26        0
    //  no simd       31       44        0
    fn mul(self, other: FlectorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1       16        0
    //    simd4        7        7        0
    // Totals...
    // yes simd        8       23        0
    //  no simd       29       44        0
    fn mul(self, other: FlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for AntiCircleRotorAligningOrigin {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       14        0
    fn mul(self, other: Horizon) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Infinity> for AntiCircleRotorAligningOrigin {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn mul(self, other: Infinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       43        0
    //    simd3        0        6        0
    //    simd4        6        0        0
    // Totals...
    // yes simd       26       49        0
    //  no simd       44       61        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineAtInfinity> for AntiCircleRotorAligningOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       21        0
    //    simd3        0        3        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        9       24        0
    //  no simd       18       30        0
    fn mul(self, other: LineAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       20        0
    //    simd3        0        2        0
    //    simd4        3        1        0
    // Totals...
    // yes simd        9       23        0
    //  no simd       18       30        0
    fn mul(self, other: LineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       53        0
    //    simd3        0        4        0
    //    simd4        8        4        0
    // Totals...
    // yes simd       40       61        0
    //  no simd       64       81        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorAtInfinity> for AntiCircleRotorAligningOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       16       28        0
    //  no simd       28       40        0
    fn mul(self, other: MotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       16       28        0
    //  no simd       28       40        0
    fn mul(self, other: MotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for AntiCircleRotorAligningOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       54       81        0
    //    simd2        9       10        0
    //    simd3       36       49        0
    //    simd4       27       18        0
    // Totals...
    // yes simd      126      158        0
    //  no simd      288      320        0
    fn mul(self, other: MultiVector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircle> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       17        0
    //    simd3        2        4        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       11       24        0
    //  no simd       24       41        0
    fn mul(self, other: MysteryCircle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircleRotor> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       20        0
    //    simd3        3        5        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       16       29        0
    //  no simd       34       51        0
    fn mul(self, other: MysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipole> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       17        0
    //    simd3        2        4        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       11       24        0
    //  no simd       24       41        0
    fn mul(self, other: MysteryDipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipoleInversion> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       32        0
    //    simd3        0        5        0
    //    simd4       11        6        0
    // Totals...
    // yes simd       24       43        0
    //  no simd       57       71        0
    fn mul(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorEven> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       34        0
    //    simd3        0        2        0
    //    simd4       12       10        0
    // Totals...
    // yes simd       32       46        0
    //  no simd       68       80        0
    fn mul(self, other: MysteryVersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorOdd> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       36        0
    //    simd3        0        3        0
    //    simd4       12        9        0
    // Totals...
    // yes simd       31       48        0
    //  no simd       67       81        0
    fn mul(self, other: MysteryVersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullCircleAtOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       23        0
    //    simd3        0        2        0
    //    simd4        3        1        0
    // Totals...
    // yes simd        9       26        0
    //  no simd       18       33        0
    fn mul(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleAtOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       17        0
    //    simd3        1        3        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       10       21        0
    //  no simd       18       30        0
    fn mul(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleInversionAtOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       23        0
    //    simd3        0        3        0
    //    simd4        4        2        0
    // Totals...
    // yes simd       18       28        0
    //  no simd       30       40        0
    fn mul(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullSphereAtOrigin> for AntiCircleRotorAligningOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn mul(self, other: NullSphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullVersorEvenAtOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       20        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       13       26        0
    //  no simd       31       44        0
    fn mul(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Origin> for AntiCircleRotorAligningOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        4        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       13        0
    fn mul(self, other: Origin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for AntiCircleRotorAligningOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       19        0
    //    simd3        1        3        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       14       25        0
    //  no simd       25       40        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<PlaneOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       20        0
    //    simd3        0        3        0
    //    simd4        4        1        0
    // Totals...
    // yes simd        6       24        0
    //  no simd       18       33        0
    fn mul(self, other: PlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for AntiCircleRotorAligningOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       26        0
    //    simd3        2        4        0
    //    simd4        5        4        0
    // Totals...
    // yes simd       16       34        0
    //  no simd       35       54        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPointAtOrigin> for AntiCircleRotorAligningOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        2        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        6       20        0
    fn mul(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn mul(self, other: Scalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for AntiCircleRotorAligningOrigin {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for AntiCircleRotorAligningOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       22        0
    //    simd3        2        4        0
    //    simd4        5        4        0
    // Totals...
    // yes simd       16       30        0
    //  no simd       35       50        0
    fn mul(self, other: Sphere) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereAtOrigin> for AntiCircleRotorAligningOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        2        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        6       20        0
    fn mul(self, other: SphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       11        0
    //    simd3        2        4        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       10       20        0
    //  no simd       29       43        0
    fn mul(self, other: SphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       75        0
    //    simd3        0       11        0
    //    simd4       24       13        0
    // Totals...
    // yes simd       72       99        0
    //  no simd      144      160        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAligningOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       65        0
    //    simd3        0        9        0
    //    simd4       16        7        0
    // Totals...
    // yes simd       57       81        0
    //  no simd      105      120        0
    fn mul(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtInfinity> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       65        0
    //    simd3        0        8        0
    //    simd4       16        8        0
    // Totals...
    // yes simd       56       81        0
    //  no simd      104      121        0
    fn mul(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       39        0
    //    simd3        0        6        0
    //    simd4       12        6        0
    // Totals...
    // yes simd       29       51        0
    //  no simd       65       81        0
    fn mul(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       41        0
    //    simd3        0        1        0
    //    simd4       10        9        0
    // Totals...
    // yes simd       38       51        0
    //  no simd       68       80        0
    fn mul(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOrthogonalOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       53        0
    //    simd3        0        9        0
    //    simd4       19       10        0
    // Totals...
    // yes simd       51       72        0
    //  no simd      108      120        0
    fn mul(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       73        0
    //    simd3        0        9        0
    //    simd4       24       15        0
    // Totals...
    // yes simd       72       97        0
    //  no simd      144      160        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddAtInfinity> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       48        0
    //    simd3        0        7        0
    //    simd4       20       13        0
    // Totals...
    // yes simd       47       68        0
    //  no simd      107      121        0
    fn mul(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddOrthogonalOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       63        0
    //    simd3        0        7        0
    //    simd4       16        9        0
    // Totals...
    // yes simd       57       79        0
    //  no simd      105      120        0
    fn mul(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Neg for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn neg(self) -> Self::Output {
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Not for AntiCircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiCircleOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn sub(self, other: AntiCircleOnOrigin) -> Self::Output {
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0() - other.group0(),
            // e23, e31, e12
            self.group1() - other.group1(),
            // e15, e25, e35, scalar
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiCircleOnOrigin> for AntiCircleRotorAligningOrigin {
    fn sub_assign(&mut self, other: AntiCircleOnOrigin) {
        *self = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0() - other.group0(),
            // e23, e31, e12
            self.group1() - other.group1(),
            // e15, e25, e35, scalar
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiCircleRotor> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd       10        4        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0() - other.group0(),
            // e23, e31, e12, e45
            (self.group1() - other.group1().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            self.group2() - other.group2(),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOrigin> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn sub(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0() - other.group0(),
            // e23, e31, e12
            self.group1() - other.group1(),
            // e15, e25, e35, scalar
            self.group2() - other.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiCircleRotorAligningOrigin> for AntiCircleRotorAligningOrigin {
    fn sub_assign(&mut self, other: AntiCircleRotorAligningOrigin) {
        *self = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0() - other.group0(),
            // e23, e31, e12
            self.group1() - other.group1(),
            // e15, e25, e35, scalar
            self.group2() - other.group2(),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOriginAtInfinity> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn sub(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1() - other.group0(),
            // e15, e25, e35, scalar
            self.group2() - other.group1(),
        );
    }
}
impl std::ops::SubAssign<AntiCircleRotorAligningOriginAtInfinity> for AntiCircleRotorAligningOrigin {
    fn sub_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        *self = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1() - other.group0(),
            // e15, e25, e35, scalar
            self.group2() - other.group1(),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAtInfinity> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        7        4        0
    fn sub(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            (self.group1() - other.group0().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            self.group2() - other.group1(),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        1        0
    //  no simd       10        1        0
    fn sub(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0() - other.group0().xyz(),
            // e23, e31, e12
            self.group1() - other.group1(),
            // e15, e25, e35, scalar
            self.group2() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
        );
    }
}
impl std::ops::SubAssign<AntiCircleRotorOnOrigin> for AntiCircleRotorAligningOrigin {
    fn sub_assign(&mut self, other: AntiCircleRotorOnOrigin) {
        use crate::elements::*;
        *self = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0() - other.group0().xyz(),
            // e23, e31, e12
            self.group1() - other.group1(),
            // e15, e25, e35, scalar
            self.group2() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversion> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionAtInfinity> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOnOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOrthogonalOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            (other.group1() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiDipoleOnOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiDualNum> for AntiCircleRotorAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        1        4        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar] - other[scalar]),
            // e23, e31, e12, e3215
            self.group1().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(other[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<AntiFlatOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiFlatPoint> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiFlector> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiFlectorOnOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiLine> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        7        3        0
    fn sub(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1() - other.group0(),
            // e15, e25, e35, scalar
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiLine> for AntiCircleRotorAligningOrigin {
    fn sub_assign(&mut self, other: AntiLine) {
        use crate::elements::*;
        *self = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1() - other.group0(),
            // e15, e25, e35, scalar
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<AntiLineOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn sub(self, other: AntiLineOnOrigin) -> Self::Output {
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1() - other.group0(),
            // e15, e25, e35, scalar
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiLineOnOrigin> for AntiCircleRotorAligningOrigin {
    fn sub_assign(&mut self, other: AntiLineOnOrigin) {
        *self = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1() - other.group0(),
            // e15, e25, e35, scalar
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiMotor> for AntiCircleRotorAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        7        4        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar] - other[scalar]),
            // e23, e31, e12, e3215
            (self.group1() - other.group0().xyz()).with_w(other[e3215]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            (self.group2().xyz() - other.group1().xyz()).with_w(0.0),
        );
    }
}
impl std::ops::Sub<AntiMotorOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        7        1        0
    fn sub(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1() - other.group0().xyz(),
            // e15, e25, e35, scalar
            self.group2() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
        );
    }
}
impl std::ops::SubAssign<AntiMotorOnOrigin> for AntiCircleRotorAligningOrigin {
    fn sub_assign(&mut self, other: AntiMotorOnOrigin) {
        use crate::elements::*;
        *self = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1() - other.group0().xyz(),
            // e15, e25, e35, scalar
            self.group2() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
        );
    }
}
impl std::ops::Sub<AntiMysteryCircleRotor> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotor;
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
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            (self.group1() - other.group0().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            self.group2() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
        );
    }
}
impl std::ops::Sub<AntiMysteryDipoleInversion> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiPlane> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiPlaneOnOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiScalar> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiSphereOnOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiVersorEvenOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        7        4        0
    fn sub(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar]) - other.group0(),
            // e23, e31, e12, e3215
            (self.group1() - other.group1().xyz()).with_w(0.0),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(other[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<Circle> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<CircleAligningOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            (other.group1() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<CircleAtInfinity> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<CircleAtOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<CircleOnOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            (other.group1() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<CircleOrthogonalOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<CircleRotor> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            (other.group1() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOriginAtInfinity> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<CircleRotorAtInfinity> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<CircleRotorOnOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            (other.group1() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Dipole> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        2        0        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        3        4        0
    //  no simd       10        7        0
    fn sub(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0() - other.group0(),
            // e23, e31, e12, e45
            (self.group1() - other.group1().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<DipoleAligningOrigin> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        7        4        0
    fn sub(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0() - other.group0().xyz(),
            // e23, e31, e12, e45
            self.group1().with_w(other[e45] * -1.0),
            // e15, e25, e35, scalar
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<DipoleAtInfinity> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotor;
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
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            (self.group1() - other.group0().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<DipoleAtOrigin> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        7        3        0
    fn sub(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0() - other.group0(),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::SubAssign<DipoleAtOrigin> for AntiCircleRotorAligningOrigin {
    fn sub_assign(&mut self, other: DipoleAtOrigin) {
        use crate::elements::*;
        *self = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0() - other.group0(),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<DipoleInversion> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        2        0        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        3        6        0
    //  no simd       10       15        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0().with_w(self[scalar]),
            // e23, e31, e12, e45
            (self.group1() - other.group1().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            (self.group2().xyz() - other.group2().xyz()).with_w(other[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<DipoleInversionAligningOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        1        0        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        7       12        0
    fn sub(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0().with_w(self[scalar]),
            // e23, e31, e12, e45
            self.group1().with_w(other[e45] * -1.0),
            // e15, e25, e35, e1234
            (self.group2().xyz() - other.group1().xyz()).with_w(other[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group2() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<DipoleInversionAtInfinity> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        6        8        0
    fn sub(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar]),
            // e23, e31, e12, e45
            (self.group1() - other.group0().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            (self.group2().xyz() - other.group1()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            other.group2() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<DipoleInversionAtOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        1        0        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        7        8        0
    fn sub(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0().with_w(self[scalar]),
            // e23, e31, e12, e3215
            self.group1().with_w(other[e3215] * -1.0),
            // e15, e25, e35, e1234
            (self.group2().xyz() - other.group1().xyz()).with_w(other[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<DipoleInversionOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        4       11        0
    fn sub(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0().with_w(self[scalar]),
            // e23, e31, e12, e45
            self.group1().with_w(other[e45] * -1.0),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(other[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            (other.group1().yzw() * Simd32x3::from(-1.0)).with_w(0.0),
        );
    }
}
impl std::ops::Sub<DipoleInversionOrthogonalOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        2        0        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3        5        0
    //  no simd       10       11        0
    fn sub(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0().with_w(self[scalar]),
            // e23, e31, e12, e3215
            (self.group1() - other.group1()).with_w(other[e3215]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            (self.group2().xyz() - other.group2().xyz()).with_w(other[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<DipoleOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        3        1        0
    fn sub(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0() - other.group0().xyz(),
            // e23, e31, e12, e45
            self.group1().with_w(other[e45] * -1.0),
            // e15, e25, e35, scalar
            self.group2(),
        );
    }
}
impl std::ops::Sub<DipoleOrthogonalOrigin> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        3        0
    //  no simd       10        3        0
    fn sub(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0() - other.group0(),
            // e23, e31, e12
            self.group1() - other.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::SubAssign<DipoleOrthogonalOrigin> for AntiCircleRotorAligningOrigin {
    fn sub_assign(&mut self, other: DipoleOrthogonalOrigin) {
        use crate::elements::*;
        *self = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0() - other.group0(),
            // e23, e31, e12
            self.group1() - other.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<DualNum> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<FlatOrigin> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1().with_w(other[e45] * -1.0),
            // e15, e25, e35, scalar
            self.group2(),
        );
    }
}
impl std::ops::Sub<FlatPoint> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        4        4        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1().with_w(other[e45] * -1.0),
            // e15, e25, e35, scalar
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<FlatPointAtInfinity> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        4        3        0
    fn sub(self, other: FlatPointAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::SubAssign<FlatPointAtInfinity> for AntiCircleRotorAligningOrigin {
    fn sub_assign(&mut self, other: FlatPointAtInfinity) {
        use crate::elements::*;
        *self = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<Flector> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        3        5        0
    fn sub(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar]),
            // e23, e31, e12, e45
            self.group1().with_w(other[e45] * -1.0),
            // e15, e25, e35, e1234
            (self.group2().xyz() - other.group0().xyz()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<FlectorAtInfinity> for AntiCircleRotorAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        3        1        0
    fn sub(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar]),
            // e23, e31, e12, e3215
            self.group1().with_w(other[e3215] * -1.0),
            // e15, e25, e35, e1234
            (self.group2().xyz() - other.group0().xyz()).with_w(0.0),
        );
    }
}
impl std::ops::Sub<FlectorOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar]),
            // e23, e31, e12, e45
            self.group1().with_w(other[e45] * -1.0),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            (other.group0().yzw() * Simd32x3::from(-1.0)).with_w(0.0),
        );
    }
}
impl std::ops::Sub<Horizon> for AntiCircleRotorAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar]),
            // e23, e31, e12, e3215
            self.group1().with_w(other[e3215] * -1.0),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(0.0),
        );
    }
}
impl std::ops::Sub<Infinity> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Line> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<LineAtInfinity> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<LineOnOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Motor> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MotorAtInfinity> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MotorOnOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MultiVector> for AntiCircleRotorAligningOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd2        0        1        0
    //    simd3        3        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        4        9        0
    //  no simd       10       26        0
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
            (self.group0() - other.group3().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            self.group2().xyz() - other.group4(),
            // e23, e31, e12
            self.group1() - other.group5(),
            // e415, e425, e435, e321
            other.group6() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group7() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group8() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            other.group9() * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<MysteryCircle> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MysteryCircleRotor> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MysteryDipole> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        3        4        0
    fn sub(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            (self.group1() - other.group0().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            self.group2(),
        );
    }
}
impl std::ops::Sub<MysteryDipoleInversion> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        3        7        0
    fn sub(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar]),
            // e23, e31, e12, e45
            (self.group1() - other.group0().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            (other.group1() * Simd32x3::from(-1.0)).with_w(0.0),
        );
    }
}
impl std::ops::Sub<MysteryVersorEven> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MysteryVersorOdd> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        4        7        0
    fn sub(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar] - other[scalar]),
            // e23, e31, e12, e45
            (self.group1() - other.group1().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            (other.group0().yzw() * Simd32x3::from(-1.0)).with_w(0.0),
        );
    }
}
impl std::ops::Sub<NullCircleAtOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullDipoleAtOrigin> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn sub(self, other: NullDipoleAtOrigin) -> Self::Output {
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0() - other.group0(),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, scalar
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<NullDipoleAtOrigin> for AntiCircleRotorAligningOrigin {
    fn sub_assign(&mut self, other: NullDipoleAtOrigin) {
        *self = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0() - other.group0(),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, scalar
            self.group2(),
        );
    }
}
impl std::ops::Sub<NullDipoleInversionAtOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        4        7        0
    fn sub(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0().with_w(self[scalar]),
            // e23, e31, e12, e3215
            self.group1().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(other[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<NullSphereAtOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar]),
            // e23, e31, e12, e3215
            self.group1().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(other[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<NullVersorEvenAtOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Origin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Plane> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar]),
            // e23, e31, e12, e45
            self.group1().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            other.group0() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<PlaneOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar]),
            // e23, e31, e12, e45
            self.group1().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
        );
    }
}
impl std::ops::Sub<RoundPoint> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<RoundPointAtOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Scalar> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, scalar
            self.group2() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
        );
    }
}
impl std::ops::SubAssign<Scalar> for AntiCircleRotorAligningOrigin {
    fn sub_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, scalar
            self.group2() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
        );
    }
}
impl std::ops::Sub<Sphere> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar]),
            // e23, e31, e12, e45
            self.group1().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(other[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group0() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<SphereAtOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar]),
            // e23, e31, e12, e3215
            self.group1().with_w(other[e3215] * -1.0),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(other[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<SphereOnOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar]),
            // e23, e31, e12, e45
            self.group1().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(other[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
        );
    }
}
impl std::ops::Sub<VersorEven> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<VersorEvenAligningOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<VersorEvenAtInfinity> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<VersorEvenAtOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<VersorEvenOnOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<VersorEvenOrthogonalOrigin> for AntiCircleRotorAligningOrigin {
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
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group2().xyz(),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<VersorOdd> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        3        3        0
    //  no simd       10       12        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar]) - other.group0(),
            // e23, e31, e12, e45
            (self.group1() - other.group1().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            (self.group2().xyz() - other.group2().xyz()).with_w(other[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<VersorOddAtInfinity> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        7        8        0
    fn sub(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar] - other[scalar]),
            // e23, e31, e12, e45
            (self.group1() - other.group1().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            (self.group2().xyz() - other.group0().yzw()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            other.group2() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<VersorOddOrthogonalOrigin> for AntiCircleRotorAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3        2        0
    //  no simd       10        8        0
    fn sub(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(self[scalar]) - other.group0(),
            // e23, e31, e12, e3215
            (self.group1() - other.group1().xyz()).with_w(other[e3215]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            (self.group2().xyz() - other.group2().xyz()).with_w(other[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}

impl TryFrom<AntiCircleRotor> for AntiCircleRotorAligningOrigin {
    type Error = String;
    fn try_from(anti_circle_rotor: AntiCircleRotor) -> Result<Self, Self::Error> {
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
            let mut error = "Elements from AntiCircleRotor do not fit into AntiCircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            anti_circle_rotor.group0(),
            // e23, e31, e12
            anti_circle_rotor.group1().xyz(),
            // e15, e25, e35, scalar
            anti_circle_rotor.group2(),
        ));
    }
}

impl TryFrom<AntiCircleRotorAtInfinity> for AntiCircleRotorAligningOrigin {
    type Error = String;
    fn try_from(anti_circle_rotor_at_infinity: AntiCircleRotorAtInfinity) -> Result<Self, Self::Error> {
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
            let mut error = "Elements from AntiCircleRotorAtInfinity do not fit into AntiCircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            anti_circle_rotor_at_infinity.group0().xyz(),
            // e15, e25, e35, scalar
            anti_circle_rotor_at_infinity.group1(),
        ));
    }
}

impl TryFrom<AntiDualNum> for AntiCircleRotorAligningOrigin {
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
            let mut error = "Elements from AntiDualNum do not fit into AntiCircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, scalar
            Simd32x3::from(0.0).with_w(anti_dual_num[scalar]),
        ));
    }
}

impl TryFrom<AntiMotor> for AntiCircleRotorAligningOrigin {
    type Error = String;
    fn try_from(anti_motor: AntiMotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_motor[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMotor do not fit into AntiCircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            anti_motor.group0().xyz(),
            // e15, e25, e35, scalar
            Simd32x4::from([anti_motor[e15], anti_motor[e25], anti_motor[e35], anti_motor[scalar]]),
        ));
    }
}

impl TryFrom<AntiMysteryCircleRotor> for AntiCircleRotorAligningOrigin {
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
            let mut error = "Elements from AntiMysteryCircleRotor do not fit into AntiCircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            anti_mystery_circle_rotor.group0().xyz(),
            // e15, e25, e35, scalar
            Simd32x3::from(0.0).with_w(anti_mystery_circle_rotor[scalar]),
        ));
    }
}

impl TryFrom<AntiVersorEvenOnOrigin> for AntiCircleRotorAligningOrigin {
    type Error = String;
    fn try_from(anti_versor_even_on_origin: AntiVersorEvenOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_versor_even_on_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiVersorEvenOnOrigin do not fit into AntiCircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            anti_versor_even_on_origin.group0().xyz(),
            // e23, e31, e12
            anti_versor_even_on_origin.group1().xyz(),
            // e15, e25, e35, scalar
            Simd32x3::from(0.0).with_w(anti_versor_even_on_origin[scalar]),
        ));
    }
}

impl TryFrom<Dipole> for AntiCircleRotorAligningOrigin {
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
            let mut error = "Elements from Dipole do not fit into AntiCircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            dipole.group0(),
            // e23, e31, e12
            dipole.group1().xyz(),
            // e15, e25, e35, scalar
            dipole.group2().with_w(0.0),
        ));
    }
}

impl TryFrom<DipoleAligningOrigin> for AntiCircleRotorAligningOrigin {
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
            let mut error = "Elements from DipoleAligningOrigin do not fit into AntiCircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            dipole_aligning_origin.group0().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, scalar
            dipole_aligning_origin.group1().with_w(0.0),
        ));
    }
}

impl TryFrom<DipoleAtInfinity> for AntiCircleRotorAligningOrigin {
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
            let mut error = "Elements from DipoleAtInfinity do not fit into AntiCircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            dipole_at_infinity.group0().xyz(),
            // e15, e25, e35, scalar
            dipole_at_infinity.group1().with_w(0.0),
        ));
    }
}

impl TryFrom<DipoleInversion> for AntiCircleRotorAligningOrigin {
    type Error = String;
    fn try_from(dipole_inversion: DipoleInversion) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
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
        let el = dipole_inversion[14];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversion do not fit into AntiCircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            dipole_inversion.group0(),
            // e23, e31, e12
            dipole_inversion.group1().xyz(),
            // e15, e25, e35, scalar
            dipole_inversion.group2().xyz().with_w(0.0),
        ));
    }
}

impl TryFrom<DipoleInversionAligningOrigin> for AntiCircleRotorAligningOrigin {
    type Error = String;
    fn try_from(dipole_inversion_aligning_origin: DipoleInversionAligningOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_aligning_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_aligning_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
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
        let el = dipole_inversion_aligning_origin[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversionAligningOrigin do not fit into AntiCircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            dipole_inversion_aligning_origin.group0().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, scalar
            dipole_inversion_aligning_origin.group1().xyz().with_w(0.0),
        ));
    }
}

impl TryFrom<DipoleInversionAtInfinity> for AntiCircleRotorAligningOrigin {
    type Error = String;
    fn try_from(dipole_inversion_at_infinity: DipoleInversionAtInfinity) -> Result<Self, Self::Error> {
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
        let el = dipole_inversion_at_infinity[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversionAtInfinity do not fit into AntiCircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            dipole_inversion_at_infinity.group0().xyz(),
            // e15, e25, e35, scalar
            dipole_inversion_at_infinity.group1().with_w(0.0),
        ));
    }
}

impl TryFrom<DipoleInversionAtOrigin> for AntiCircleRotorAligningOrigin {
    type Error = String;
    fn try_from(dipole_inversion_at_origin: DipoleInversionAtOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_at_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversionAtOrigin do not fit into AntiCircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            dipole_inversion_at_origin.group0().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, scalar
            dipole_inversion_at_origin.group1().xyz().with_w(0.0),
        ));
    }
}

impl TryFrom<DipoleInversionOnOrigin> for AntiCircleRotorAligningOrigin {
    type Error = String;
    fn try_from(dipole_inversion_on_origin: DipoleInversionOnOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_on_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
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
            let mut error = "Elements from DipoleInversionOnOrigin do not fit into AntiCircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            dipole_inversion_on_origin.group0().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, scalar
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<DipoleInversionOrthogonalOrigin> for AntiCircleRotorAligningOrigin {
    type Error = String;
    fn try_from(dipole_inversion_orthogonal_origin: DipoleInversionOrthogonalOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_orthogonal_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_orthogonal_origin[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversionOrthogonalOrigin do not fit into AntiCircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            dipole_inversion_orthogonal_origin.group0().xyz(),
            // e23, e31, e12
            dipole_inversion_orthogonal_origin.group1(),
            // e15, e25, e35, scalar
            dipole_inversion_orthogonal_origin.group2().xyz().with_w(0.0),
        ));
    }
}

impl TryFrom<DipoleOnOrigin> for AntiCircleRotorAligningOrigin {
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
            let mut error = "Elements from DipoleOnOrigin do not fit into AntiCircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            dipole_on_origin.group0().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, scalar
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<FlatPoint> for AntiCircleRotorAligningOrigin {
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
            let mut error = "Elements from FlatPoint do not fit into AntiCircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, scalar
            flat_point.group0().xyz().with_w(0.0),
        ));
    }
}

impl TryFrom<Flector> for AntiCircleRotorAligningOrigin {
    type Error = String;
    fn try_from(flector: Flector) -> Result<Self, Self::Error> {
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
        let el = flector[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Flector do not fit into AntiCircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, scalar
            flector.group0().xyz().with_w(0.0),
        ));
    }
}

impl TryFrom<FlectorAtInfinity> for AntiCircleRotorAligningOrigin {
    type Error = String;
    fn try_from(flector_at_infinity: FlectorAtInfinity) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = flector_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from FlectorAtInfinity do not fit into AntiCircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, scalar
            flector_at_infinity.group0().xyz().with_w(0.0),
        ));
    }
}

impl TryFrom<MultiVector> for AntiCircleRotorAligningOrigin {
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
            let mut error = "Elements from MultiVector do not fit into AntiCircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            multi_vector.group3().xyz(),
            // e23, e31, e12
            multi_vector.group5(),
            // e15, e25, e35, scalar
            multi_vector.group4().with_w(multi_vector[scalar]),
        ));
    }
}

impl TryFrom<MysteryDipole> for AntiCircleRotorAligningOrigin {
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
            let mut error = "Elements from MysteryDipole do not fit into AntiCircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            mystery_dipole.group0().xyz(),
            // e15, e25, e35, scalar
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<MysteryDipoleInversion> for AntiCircleRotorAligningOrigin {
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
            let mut error = "Elements from MysteryDipoleInversion do not fit into AntiCircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            mystery_dipole_inversion.group0().xyz(),
            // e15, e25, e35, scalar
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<MysteryVersorOdd> for AntiCircleRotorAligningOrigin {
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
            let mut error = "Elements from MysteryVersorOdd do not fit into AntiCircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            mystery_versor_odd.group1().xyz(),
            // e15, e25, e35, scalar
            Simd32x3::from(0.0).with_w(mystery_versor_odd[scalar]),
        ));
    }
}

impl TryFrom<NullDipoleInversionAtOrigin> for AntiCircleRotorAligningOrigin {
    type Error = String;
    fn try_from(null_dipole_inversion_at_origin: NullDipoleInversionAtOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = null_dipole_inversion_at_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from NullDipoleInversionAtOrigin do not fit into AntiCircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            null_dipole_inversion_at_origin.group0().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, scalar
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<VersorOdd> for AntiCircleRotorAligningOrigin {
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
        let el = versor_odd[15];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOdd do not fit into AntiCircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            versor_odd.group0().xyz(),
            // e23, e31, e12
            versor_odd.group1().xyz(),
            // e15, e25, e35, scalar
            Simd32x4::from([versor_odd[e15], versor_odd[e25], versor_odd[e35], versor_odd[scalar]]),
        ));
    }
}

impl TryFrom<VersorOddAtInfinity> for AntiCircleRotorAligningOrigin {
    type Error = String;
    fn try_from(versor_odd_at_infinity: VersorOddAtInfinity) -> Result<Self, Self::Error> {
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
        let el = versor_odd_at_infinity[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOddAtInfinity do not fit into AntiCircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            versor_odd_at_infinity.group1().xyz(),
            // e15, e25, e35, scalar
            versor_odd_at_infinity.group0().yzwx(),
        ));
    }
}

impl TryFrom<VersorOddOrthogonalOrigin> for AntiCircleRotorAligningOrigin {
    type Error = String;
    fn try_from(versor_odd_orthogonal_origin: VersorOddOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd_orthogonal_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_orthogonal_origin[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOddOrthogonalOrigin do not fit into AntiCircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            versor_odd_orthogonal_origin.group0().xyz(),
            // e23, e31, e12
            versor_odd_orthogonal_origin.group1().xyz(),
            // e15, e25, e35, scalar
            Simd32x4::from([
                versor_odd_orthogonal_origin[e15],
                versor_odd_orthogonal_origin[e25],
                versor_odd_orthogonal_origin[e35],
                versor_odd_orthogonal_origin[scalar],
            ]),
        ));
    }
}
