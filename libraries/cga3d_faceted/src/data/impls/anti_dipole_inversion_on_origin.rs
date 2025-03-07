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
//   Median:         1       2       0
//  Average:         6      10       0
//  Maximum:        95     118       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         1       4       0
//  Average:        13      18       0
//  Maximum:       228     256       0
impl std::ops::Add<AntiCircleOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiCircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotor> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOrigin> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOriginAtInfinity> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorAtInfinity> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiDipoleInversion> for AntiDipoleInversionOnOrigin {
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
            other.group2() + Simd32x3::from(0.0).with_w(self[e4]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], 0.0]) + other.group3(),
        );
    }
}
impl std::ops::Add<AntiDipoleInversionAtInfinity> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            other.group0() + Simd32x3::from(0.0).with_w(self[e321]),
            // e235, e315, e125, e4
            other.group1().with_w(self[e4]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], 0.0]) + other.group2(),
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            other.group0() + self.group0(),
            // e4, e1, e2, e3
            other.group1() + self.group1(),
        );
    }
}
impl std::ops::AddAssign<AntiDipoleInversionOnOrigin> for AntiDipoleInversionOnOrigin {
    fn add_assign(&mut self, other: AntiDipoleInversionOnOrigin) {
        *self = AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            other.group0() + self.group0(),
            // e4, e1, e2, e3
            other.group1() + self.group1(),
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOrthogonalOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz() + other.group0().xyz(),
            // e415, e425, e435, e321
            other.group1().with_w(self[e321]),
            // e235, e315, e125, e4
            other.group2().xyz().with_w(self[e4] + other[e4]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], other[e5]]),
        );
    }
}
impl std::ops::Add<AntiDipoleOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return AntiDipoleInversionOnOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0() + other.group0(), /* e4, e1, e2, e3 */ self.group1());
    }
}
impl std::ops::AddAssign<AntiDipoleOnOrigin> for AntiDipoleInversionOnOrigin {
    fn add_assign(&mut self, other: AntiDipoleOnOrigin) {
        *self = AntiDipoleInversionOnOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0() + other.group0(), /* e4, e1, e2, e3 */ self.group1());
    }
}
impl std::ops::Add<AntiDualNum> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiFlatOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321]),
            // e4, e1, e2, e3
            self.group1(),
        );
    }
}
impl std::ops::AddAssign<AntiFlatOrigin> for AntiDipoleInversionOnOrigin {
    fn add_assign(&mut self, other: AntiFlatOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321]),
            // e4, e1, e2, e3
            self.group1(),
        );
    }
}
impl std::ops::Add<AntiFlatPoint> for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321]),
            // e235, e315, e125, e5
            other.group0().xyz().with_w(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
        );
    }
}
impl std::ops::Add<AntiFlector> for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235], other[e315], other[e125], other[e5]]),
            // e1, e2, e3, e4
            self.group1().yzwx() + other.group1().xyz().with_w(0.0),
        );
    }
}
impl std::ops::Add<AntiFlectorOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321]),
            // e4, e1, e2, e3
            Simd32x4::from([0.0, other[e1], other[e2], other[e3]]) + self.group1(),
        );
    }
}
impl std::ops::AddAssign<AntiFlectorOnOrigin> for AntiDipoleInversionOnOrigin {
    fn add_assign(&mut self, other: AntiFlectorOnOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321]),
            // e4, e1, e2, e3
            Simd32x4::from([0.0, other[e1], other[e2], other[e3]]) + self.group1(),
        );
    }
}
impl std::ops::Add<AntiLine> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiLineOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiLineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiMotor> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<AntiMotorOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiMysteryCircleRotor> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiMysteryDipoleInversion> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            other.group0().xyz().with_w(self[e321] + other[e321]),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e1, e2, e3, e5
            (other.group1() + self.group1().yzw()).with_w(0.0),
        );
    }
}
impl std::ops::Add<AntiPlane> for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(other[e5]),
            // e1, e2, e3, e4
            self.group1().yzwx() + other.group0().xyz().with_w(0.0),
        );
    }
}
impl std::ops::Add<AntiPlaneOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: AntiPlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e4, e1, e2, e3
            Simd32x4::from([0.0, other[e1], self[e2], self[e3]]) + self.group1().xy().with_zw(other[e2], other[e3]),
        );
    }
}
impl std::ops::AddAssign<AntiPlaneOnOrigin> for AntiDipoleInversionOnOrigin {
    fn add_assign(&mut self, other: AntiPlaneOnOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e4, e1, e2, e3
            Simd32x4::from([0.0, other[e1], self[e2], self[e3]]) + self.group1().xy().with_zw(other[e2], other[e3]),
        );
    }
}
impl std::ops::Add<AntiScalar> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
        );
    }
}
impl std::ops::Add<AntiSphereOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: AntiSphereOnOrigin) -> Self::Output {
        return AntiDipoleInversionOnOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0(), /* e4, e1, e2, e3 */ self.group1() + other.group0().wxyz());
    }
}
impl std::ops::AddAssign<AntiSphereOnOrigin> for AntiDipoleInversionOnOrigin {
    fn add_assign(&mut self, other: AntiSphereOnOrigin) {
        *self = AntiDipoleInversionOnOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0(), /* e4, e1, e2, e3 */ self.group1() + other.group0().wxyz());
    }
}
impl std::ops::Add<AntiVersorEvenOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Circle> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() + self.group0().xyz(),
            // e415, e425, e435, e321
            other.group1().xyz().with_w(self[e321] + other[e321]),
            // e235, e315, e125, e4
            other.group2().with_w(self[e4]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], 0.0]),
        );
    }
}
impl std::ops::Add<CircleAligningOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: CircleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() + self.group0().xyz(),
            // e415, e425, e435, e321
            other.group1().with_w(self[e321]),
            // e235, e315, e125, e4
            other.group2().with_w(self[e4]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], 0.0]),
        );
    }
}
impl std::ops::Add<CircleAtInfinity> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: CircleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            other.group0().xyz().with_w(self[e321] + other[e321]),
            // e235, e315, e125, e4
            other.group1().with_w(self[e4]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], 0.0]),
        );
    }
}
impl std::ops::Add<CircleAtOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: CircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            (other.group0() + self.group0().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            other.group1().with_w(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
        );
    }
}
impl std::ops::Add<CircleOnOrigin> for AntiDipoleInversionOnOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], 0.0]),
        );
    }
}
impl std::ops::Add<CircleOrthogonalOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() + other.group0(),
            // e235, e315, e125, e5
            other.group1().with_w(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
        );
    }
}
impl std::ops::Add<CircleRotor> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() + self.group0().xyz()).with_w(other[e12345]),
            // e415, e425, e435, e321
            other.group1().xyz().with_w(self[e321] + other[e321]),
            // e235, e315, e125, e5
            other.group2().xyz().with_w(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
        );
    }
}
impl std::ops::Add<CircleRotorAligningOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() + self.group0().xyz()).with_w(other[e12345]),
            // e415, e425, e435, e321
            other.group1().with_w(self[e321]),
            // e235, e315, e125, e5
            other.group2().xyz().with_w(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
        );
    }
}
impl std::ops::Add<CircleRotorAligningOriginAtInfinity> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    fn add(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]),
            // e415, e425, e435, e321
            other.group0().with_w(self[e321]),
            // e235, e315, e125, e5
            other.group1().xyz().with_w(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
        );
    }
}
impl std::ops::Add<CircleRotorAtInfinity> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: CircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]),
            // e415, e425, e435, e321
            other.group0().xyz().with_w(self[e321] + other[e321]),
            // e235, e315, e125, e5
            other.group1().xyz().with_w(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
        );
    }
}
impl std::ops::Add<CircleRotorOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: CircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0().xyz() + other.group0().xyz()).with_w(other[e12345]),
            // e415, e425, e435, e321
            other.group1().with_w(self[e321]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
        );
    }
}
impl std::ops::Add<Dipole> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleAligningOrigin> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleAtInfinity> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleAtOrigin> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleInversion> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionAligningOrigin> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionAtInfinity> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionAtOrigin> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            other.group1(),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleInversionOrthogonalOrigin> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleOrthogonalOrigin> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DualNum> for AntiDipoleInversionOnOrigin {
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
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzw().with_w(self[e4] + other[e4]),
        );
    }
}
impl std::ops::Add<FlatOrigin> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<FlatPoint> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<FlatPointAtInfinity> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: FlatPointAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Flector> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<FlectorAtInfinity> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<FlectorOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Horizon> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<Infinity> for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    fn add(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(other[e5]),
            // e1, e2, e3, e4
            self.group1().yzwx(),
        );
    }
}
impl std::ops::Add<Line> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversion;
    fn add(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            other.group0().with_w(self[e321]),
            // e235, e315, e125, e4
            other.group1().with_w(self[e4]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], 0.0]),
        );
    }
}
impl std::ops::Add<LineAtInfinity> for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    fn add(self, other: LineAtInfinity) -> Self::Output {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            other.group0().with_w(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
        );
    }
}
impl std::ops::Add<LineOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversion;
    fn add(self, other: LineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            other.group0().with_w(self[e321]),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], 0.0]),
        );
    }
}
impl std::ops::Add<Motor> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    fn add(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]),
            // e415, e425, e435, e321
            other.group0().xyz().with_w(self[e321]),
            // e235, e315, e125, e5
            other.group1(),
            // e1, e2, e3, e4
            self.group1().yzwx(),
        );
    }
}
impl std::ops::Add<MotorAtInfinity> for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    fn add(self, other: MotorAtInfinity) -> Self::Output {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            other.group0(),
            // e1, e2, e3, e4
            self.group1().yzwx(),
        );
    }
}
impl std::ops::Add<MotorOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    fn add(self, other: MotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]),
            // e415, e425, e435, e321
            other.group0().xyz().with_w(self[e321]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
        );
    }
}
impl std::ops::Add<MultiVector> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        8        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            other.group0(),
            // e1, e2, e3, e4
            other.group1() + self.group1().yzwx(),
            // e5
            other[e5],
            // e41, e42, e43, e45
            other.group3(),
            // e15, e25, e35
            other.group4(),
            // e23, e31, e12
            other.group5(),
            // e415, e425, e435, e321
            other.group6().xyz().with_w(self[e321] + other[e321]),
            // e423, e431, e412
            other.group7() + self.group0().xyz(),
            // e235, e315, e125
            other.group8(),
            // e1234, e4235, e4315, e4125
            other.group9(),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<MysteryCircle> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: MysteryCircle) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            other.group0().xyz().with_w(self[e321] + other[e321]),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], 0.0]),
        );
    }
}
impl std::ops::Add<MysteryCircleRotor> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]),
            // e415, e425, e435, e321
            other.group0().xyz().with_w(self[e321] + other[e321]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
        );
    }
}
impl std::ops::Add<MysteryDipole> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MysteryDipoleInversion> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MysteryVersorEven> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: MysteryVersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]),
            // e415, e425, e435, e321
            other.group1().xyz().with_w(self[e321] + other[e321]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]) + self.group1().yzwx(),
        );
    }
}
impl std::ops::Add<MysteryVersorOdd> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullCircleAtOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: NullCircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            (other.group0() + self.group0().xyz()).with_w(self[e321]),
            // e4, e1, e2, e3
            self.group1(),
        );
    }
}
impl std::ops::AddAssign<NullCircleAtOrigin> for AntiDipoleInversionOnOrigin {
    fn add_assign(&mut self, other: NullCircleAtOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            (other.group0() + self.group0().xyz()).with_w(self[e321]),
            // e4, e1, e2, e3
            self.group1(),
        );
    }
}
impl std::ops::Add<NullDipoleAtOrigin> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: NullDipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullDipoleInversionAtOrigin> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullSphereAtOrigin> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullVersorEvenAtOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() + other.group0().xyz().with_w(0.0),
            // e4, e1, e2, e3
            Simd32x4::from([other[e4], 0.0, 0.0, 0.0]) + self.group1(),
        );
    }
}
impl std::ops::AddAssign<NullVersorEvenAtOrigin> for AntiDipoleInversionOnOrigin {
    fn add_assign(&mut self, other: NullVersorEvenAtOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() + other.group0().xyz().with_w(0.0),
            // e4, e1, e2, e3
            Simd32x4::from([other[e4], 0.0, 0.0, 0.0]) + self.group1(),
        );
    }
}
impl std::ops::Add<Origin> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e4, e1, e2, e3
            Simd32x4::from([other[e4], 0.0, 0.0, 0.0]) + self.group1(),
        );
    }
}
impl std::ops::AddAssign<Origin> for AntiDipoleInversionOnOrigin {
    fn add_assign(&mut self, other: Origin) {
        use crate::elements::*;
        *self = AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e4, e1, e2, e3
            Simd32x4::from([other[e4], 0.0, 0.0, 0.0]) + self.group1(),
        );
    }
}
impl std::ops::Add<Plane> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<PlaneOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<RoundPoint> for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(other[e5]),
            // e1, e2, e3, e4
            other.group0() + self.group1().yzwx(),
        );
    }
}
impl std::ops::Add<RoundPointAtOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(other[e5]),
            // e1, e2, e3, e4
            self.group1().yzw().with_w(self[e4] + other[e4]),
        );
    }
}
impl std::ops::Add<Scalar> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Sphere> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<SphereAtOrigin> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<SphereOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            other.group0().wxyz(),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<VersorEven> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        8        0        0
    fn add(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0().xyz() + other.group0().xyz()).with_w(other[e12345]),
            // e415, e425, e435, e321
            other.group1().xyz().with_w(self[e321] + other[e321]),
            // e235, e315, e125, e5
            other.group2(),
            // e1, e2, e3, e4
            other.group3() + self.group1().yzwx(),
        );
    }
}
impl std::ops::Add<VersorEvenAligningOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0().xyz() + other.group0().xyz()).with_w(other[e12345]),
            // e415, e425, e435, e321
            other.group1().xyz().with_w(self[e321]),
            // e235, e315, e125, e5
            other.group2(),
            // e1, e2, e3, e4
            self.group1().yzwx() + Simd32x3::from(0.0).with_w(other[e4]),
        );
    }
}
impl std::ops::Add<VersorEvenAtInfinity> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]),
            // e415, e425, e435, e321
            other.group1().xyz().with_w(self[e321] + other[e321]),
            // e235, e315, e125, e5
            other.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]) + self.group1().yzwx(),
        );
    }
}
impl std::ops::Add<VersorEvenAtOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() + other.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            other.group1(),
            // e1, e2, e3, e4
            self.group1().yzwx() + Simd32x3::from(0.0).with_w(other[e4]),
        );
    }
}
impl std::ops::Add<VersorEvenOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: VersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0().xyz() + other.group0().xyz()).with_w(other[e12345]),
            // e415, e425, e435, e321
            other.group1().xyz().with_w(self[e321]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx() + Simd32x3::from(0.0).with_w(other[e4]),
        );
    }
}
impl std::ops::Add<VersorEvenOrthogonalOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() + other.group0(),
            // e235, e315, e125, e5
            other.group1(),
            // e1, e2, e3, e4
            other.group2() + self.group1().yzwx(),
        );
    }
}
impl std::ops::Add<VersorOdd> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<VersorOddAtInfinity> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<VersorOddOrthogonalOrigin> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            other[e3215],
        );
    }
}

impl From<AntiDipoleOnOrigin> for AntiDipoleInversionOnOrigin {
    fn from(from_anti_dipole_on_origin: AntiDipoleOnOrigin) -> Self {
        return AntiDipoleInversionOnOrigin::from_groups(/* e423, e431, e412, e321 */ from_anti_dipole_on_origin.group0(), /* e4, e1, e2, e3 */ Simd32x4::from(0.0));
    }
}

impl From<AntiFlatOrigin> for AntiDipoleInversionOnOrigin {
    fn from(from_anti_flat_origin: AntiFlatOrigin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(from_anti_flat_origin[e321]),
            // e4, e1, e2, e3
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiFlectorOnOrigin> for AntiDipoleInversionOnOrigin {
    fn from(from_anti_flector_on_origin: AntiFlectorOnOrigin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(from_anti_flector_on_origin[e321]),
            // e4, e1, e2, e3
            Simd32x4::from([0.0, from_anti_flector_on_origin[e1], from_anti_flector_on_origin[e2], from_anti_flector_on_origin[e3]]),
        );
    }
}

impl From<AntiPlaneOnOrigin> for AntiDipoleInversionOnOrigin {
    fn from(from_anti_plane_on_origin: AntiPlaneOnOrigin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e4, e1, e2, e3
            Simd32x4::from([0.0, from_anti_plane_on_origin[e1], from_anti_plane_on_origin[e2], from_anti_plane_on_origin[e3]]),
        );
    }
}

impl From<AntiSphereOnOrigin> for AntiDipoleInversionOnOrigin {
    fn from(from_anti_sphere_on_origin: AntiSphereOnOrigin) -> Self {
        return AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e4, e1, e2, e3
            from_anti_sphere_on_origin.group0().wxyz(),
        );
    }
}

impl From<NullCircleAtOrigin> for AntiDipoleInversionOnOrigin {
    fn from(from_null_circle_at_origin: NullCircleAtOrigin) -> Self {
        return AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            from_null_circle_at_origin.group0().with_w(0.0),
            // e4, e1, e2, e3
            Simd32x4::from(0.0),
        );
    }
}

impl From<NullVersorEvenAtOrigin> for AntiDipoleInversionOnOrigin {
    fn from(from_null_versor_even_at_origin: NullVersorEvenAtOrigin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            from_null_versor_even_at_origin.group0().xyz().with_w(0.0),
            // e4, e1, e2, e3
            Simd32x4::from([from_null_versor_even_at_origin[e4], 0.0, 0.0, 0.0]),
        );
    }
}

impl From<Origin> for AntiDipoleInversionOnOrigin {
    fn from(from_origin: Origin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e4, e1, e2, e3
            Simd32x4::from([from_origin[e4], 0.0, 0.0, 0.0]),
        );
    }
}
impl std::ops::Mul<AntiCircleOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       26        0
    //    simd3        0        1        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       28       37        0
    fn mul(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleOnOrigin> for AntiDipoleInversionOnOrigin {
    fn mul_assign(&mut self, other: AntiCircleOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       51        0
    //    simd3        0        3        0
    //    simd4       10        7        0
    // Totals...
    // yes simd       42       61        0
    //  no simd       72       88        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       52        0
    //    simd3        0        3        0
    //    simd4        8        5        0
    // Totals...
    // yes simd       40       60        0
    //  no simd       64       81        0
    fn mul(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOriginAtInfinity> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       43        0
    //    simd3        0        2        0
    //    simd4        7        5        0
    // Totals...
    // yes simd       31       50        0
    //  no simd       52       69        0
    fn mul(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAtInfinity> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       42        0
    //    simd3        0        1        0
    //    simd4        9        8        0
    // Totals...
    // yes simd       33       51        0
    //  no simd       60       77        0
    fn mul(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       26        0
    //    simd3        0        1        0
    //    simd4        5        4        0
    // Totals...
    // yes simd       21       31        0
    //  no simd       36       45        0
    fn mul(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorOnOrigin> for AntiDipoleInversionOnOrigin {
    fn mul_assign(&mut self, other: AntiCircleRotorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for AntiDipoleInversionOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       39        0
    //    simd3        0        2        0
    //    simd4       21       19        0
    // Totals...
    // yes simd       45       60        0
    //  no simd      108      121        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionAtInfinity> for AntiDipoleInversionOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       34        0
    //    simd3        0        6        0
    //    simd4       20       14        0
    // Totals...
    // yes simd       32       54        0
    //  no simd       92      108        0
    fn mul(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       18        0
    //    simd3        0        2        0
    //    simd4        8        6        0
    // Totals...
    // yes simd       20       26        0
    //  no simd       44       48        0
    fn mul(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOrthogonalOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       39        0
    //    simd3        0        6        0
    //    simd4       14        8        0
    // Totals...
    // yes simd       36       53        0
    //  no simd       78       89        0
    fn mul(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        3        6        0
    // Totals...
    // yes simd        6       12        0
    //  no simd       12       24        0
    fn mul(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        6        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        2        8        0
    //  no simd        4       12        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiDualNum> for AntiDipoleInversionOnOrigin {
    fn mul_assign(&mut self, other: AntiDualNum) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn mul(self, other: AntiFlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for AntiDipoleInversionOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       16        0
    //    simd3        2        4        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       13       24        0
    //  no simd       29       44        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for AntiDipoleInversionOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       35        0
    //    simd3        0        3        0
    //    simd4       12        9        0
    // Totals...
    // yes simd       32       47        0
    //  no simd       68       80        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlectorOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        6        8        0
    // no simd       24       32        0
    fn mul(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       43        0
    //    simd3        0        2        0
    //    simd4        5        3        0
    // Totals...
    // yes simd       29       48        0
    //  no simd       44       61        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLineOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       17        0
    //    simd3        0        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       10       19        0
    //  no simd       16       24        0
    fn mul(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiLineOnOrigin> for AntiDipoleInversionOnOrigin {
    fn mul_assign(&mut self, other: AntiLineOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       43        0
    //    simd3        0        2        0
    //    simd4       10        8        0
    // Totals...
    // yes simd       34       53        0
    //  no simd       64       81        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotorOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       12       20        0
    //  no simd       24       32        0
    fn mul(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMotorOnOrigin> for AntiDipoleInversionOnOrigin {
    fn mul_assign(&mut self, other: AntiMotorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryCircleRotor> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       11        0
    //    simd3        4        7        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       11       21        0
    //  no simd       28       44        0
    fn mul(self, other: AntiMysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryDipoleInversion> for AntiDipoleInversionOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       21        0
    //    simd3        0        5        0
    //    simd4        9        5        0
    // Totals...
    // yes simd       21       31        0
    //  no simd       48       56        0
    fn mul(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for AntiDipoleInversionOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        5        0
    //    simd3        3        5        0
    //    simd4        3        4        0
    // Totals...
    // yes simd        8       14        0
    //  no simd       23       36        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlaneOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       12        0
    //    simd4        4        4        0
    // Totals...
    // yes simd        4       16        0
    //  no simd       16       28        0
    fn mul(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for AntiDipoleInversionOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiSphereOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       12        0
    //    simd4        4        4        0
    // Totals...
    // yes simd        8       16        0
    //  no simd       20       28        0
    fn mul(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorEvenOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       25        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       22       31        0
    //  no simd       40       49        0
    fn mul(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiVersorEvenOnOrigin> for AntiDipoleInversionOnOrigin {
    fn mul_assign(&mut self, other: AntiVersorEvenOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for AntiDipoleInversionOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       46        0
    //    simd3        0        6        0
    //    simd4       11        5        0
    // Totals...
    // yes simd       32       57        0
    //  no simd       65       84        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAligningOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       46        0
    //    simd3        0        2        0
    //    simd4        8        6        0
    // Totals...
    // yes simd       32       54        0
    //  no simd       56       76        0
    fn mul(self, other: CircleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtInfinity> for AntiDipoleInversionOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       40        0
    //    simd3        0        5        0
    //    simd4       10        5        0
    // Totals...
    // yes simd       22       50        0
    //  no simd       52       75        0
    fn mul(self, other: CircleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       30        0
    //    simd3        0        2        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       14       36        0
    //  no simd       32       52        0
    fn mul(self, other: CircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       19        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       14       24        0
    //  no simd       29       39        0
    fn mul(self, other: CircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOrthogonalOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       30        0
    //    simd3        0        4        0
    //    simd4        9        5        0
    // Totals...
    // yes simd       14       39        0
    //  no simd       41       62        0
    fn mul(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for AntiDipoleInversionOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       41        0
    //    simd3        0        4        0
    //    simd4       13        9        0
    // Totals...
    // yes simd       40       54        0
    //  no simd       79       89        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       48        0
    //    simd3        0        7        0
    //    simd4       10        3        0
    // Totals...
    // yes simd       40       58        0
    //  no simd       70       81        0
    fn mul(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOriginAtInfinity> for AntiDipoleInversionOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       39        0
    //    simd3        0        6        0
    //    simd4        9        3        0
    // Totals...
    // yes simd       31       48        0
    //  no simd       58       69        0
    fn mul(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAtInfinity> for AntiDipoleInversionOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       31        0
    //    simd3        0        2        0
    //    simd4       12       10        0
    // Totals...
    // yes simd       30       43        0
    //  no simd       66       77        0
    fn mul(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       15        0
    //    simd3        0        3        0
    //    simd4        8        5        0
    // Totals...
    // yes simd       16       23        0
    //  no simd       40       44        0
    fn mul(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       53        0
    //    simd3        0        5        0
    //    simd4        8        3        0
    // Totals...
    // yes simd       40       61        0
    //  no simd       64       80        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAligningOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       36        0
    //    simd3        0        3        0
    //    simd4        6        3        0
    // Totals...
    // yes simd       22       42        0
    //  no simd       40       57        0
    fn mul(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtInfinity> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       44        0
    //    simd3        0        3        0
    //    simd4        7        4        0
    // Totals...
    // yes simd       31       51        0
    //  no simd       52       69        0
    fn mul(self, other: DipoleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       34        0
    //    simd3        0        2        0
    //    simd4        4        2        0
    // Totals...
    // yes simd       20       38        0
    //  no simd       32       48        0
    fn mul(self, other: DipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       53        0
    //    simd3        0        4        0
    //    simd4       18       14        0
    // Totals...
    // yes simd       50       71        0
    //  no simd      104      121        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAligningOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       38        0
    //    simd3        0        2        0
    //    simd4       15       13        0
    // Totals...
    // yes simd       36       53        0
    //  no simd       81       96        0
    fn mul(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtInfinity> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       46        0
    //    simd3        0        6        0
    //    simd4       16       10        0
    // Totals...
    // yes simd       40       62        0
    //  no simd       88      104        0
    fn mul(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       34        0
    //    simd3        0        2        0
    //    simd4        8        6        0
    // Totals...
    // yes simd       24       42        0
    //  no simd       48       64        0
    fn mul(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       17        0
    //    simd3        0        1        0
    //    simd4        8        7        0
    // Totals...
    // yes simd       20       25        0
    //  no simd       44       48        0
    fn mul(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOrthogonalOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       54        0
    //    simd3        0        5        0
    //    simd4       10        5        0
    // Totals...
    // yes simd       42       64        0
    //  no simd       72       89        0
    fn mul(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        3        6        0
    // Totals...
    // yes simd        6       12        0
    //  no simd       12       24        0
    fn mul(self, other: DipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOrthogonalOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       52        0
    //    simd3        0        3        0
    //    simd4        6        3        0
    // Totals...
    // yes simd       38       58        0
    //  no simd       56       73        0
    fn mul(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for AntiDipoleInversionOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        6        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        2        8        0
    //  no simd        4       12        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn mul(self, other: FlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       21        0
    //    simd3        2        4        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       15       28        0
    //  no simd       28       45        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPointAtInfinity> for AntiDipoleInversionOnOrigin {
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
impl std::ops::Mul<Flector> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       32       44        0
    //  no simd       68       80        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorAtInfinity> for AntiDipoleInversionOnOrigin {
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
impl std::ops::Mul<FlectorOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        6        8        0
    // no simd       24       32        0
    fn mul(self, other: FlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn mul(self, other: Horizon) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Infinity> for AntiDipoleInversionOnOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        5        0
    // no simd        0       20        0
    fn mul(self, other: Infinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for AntiDipoleInversionOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       37        0
    //    simd3        0        1        0
    //    simd4        7        6        0
    // Totals...
    // yes simd       23       44        0
    //  no simd       44       64        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineAtInfinity> for AntiDipoleInversionOnOrigin {
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
impl std::ops::Mul<LineOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       13        0
    //    simd3        0        1        0
    //    simd4        4        3        0
    // Totals...
    // yes simd        4       17        0
    //  no simd       16       28        0
    fn mul(self, other: LineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for AntiDipoleInversionOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       32        0
    //    simd3        0        3        0
    //    simd4       12       10        0
    // Totals...
    // yes simd       31       45        0
    //  no simd       67       81        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorAtInfinity> for AntiDipoleInversionOnOrigin {
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
impl std::ops::Mul<MotorOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        6        8        0
    // no simd       24       32        0
    fn mul(self, other: MotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       59        0
    //    simd2        4        4        0
    //    simd3       24       31        0
    //    simd4       27       24        0
    // Totals...
    // yes simd       95      118        0
    //  no simd      228      256        0
    fn mul(self, other: MultiVector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircle> for AntiDipoleInversionOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        3        9        0
    //    simd4        2        0        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       22       36        0
    fn mul(self, other: MysteryCircle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircleRotor> for AntiDipoleInversionOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       11        0
    //    simd3        4       11        0
    //    simd4        3        0        0
    // Totals...
    // yes simd       14       22        0
    //  no simd       31       44        0
    fn mul(self, other: MysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipole> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       10        0
    //    simd3        3        6        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        8       18        0
    //  no simd       20       36        0
    fn mul(self, other: MysteryDipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipoleInversion> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       23        0
    //    simd3        0        3        0
    //    simd4        9        6        0
    // Totals...
    // yes simd       19       32        0
    //  no simd       46       56        0
    fn mul(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorEven> for AntiDipoleInversionOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       21        0
    //    simd3        0        5        0
    //    simd4       10        7        0
    // Totals...
    // yes simd       26       33        0
    //  no simd       56       64        0
    fn mul(self, other: MysteryVersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorOdd> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       24        0
    //    simd3        0        4        0
    //    simd4       10        7        0
    // Totals...
    // yes simd       23       35        0
    //  no simd       53       64        0
    fn mul(self, other: MysteryVersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullCircleAtOrigin> for AntiDipoleInversionOnOrigin {
    type Output = NullDipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn mul(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleAtOrigin> for AntiDipoleInversionOnOrigin {
    type Output = NullVersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn mul(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleInversionAtOrigin> for AntiDipoleInversionOnOrigin {
    type Output = NullVersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        6       10        0
    //  no simd       12       16        0
    fn mul(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullSphereAtOrigin> for AntiDipoleInversionOnOrigin {
    type Output = NullVersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn mul(self, other: NullSphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullVersorEvenAtOrigin> for AntiDipoleInversionOnOrigin {
    type Output = NullDipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        6       10        0
    //  no simd       12       16        0
    fn mul(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Origin> for AntiDipoleInversionOnOrigin {
    type Output = NullDipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn mul(self, other: Origin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       12        0
    //    simd3        3        5        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        8       21        0
    //  no simd       20       43        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<PlaneOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       12        0
    //    simd4        4        4        0
    // Totals...
    // yes simd        4       16        0
    //  no simd       16       28        0
    fn mul(self, other: PlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for AntiDipoleInversionOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       13        0
    //    simd3        3        6        0
    //    simd4        3        3        0
    // Totals...
    // yes simd        9       22        0
    //  no simd       24       43        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPointAtOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        7        0
    // no simd        0       28        0
    fn mul(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn mul(self, other: Scalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for AntiDipoleInversionOnOrigin {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       12        0
    //    simd3        3        5        0
    //    simd4        3        5        0
    // Totals...
    // yes simd        9       22        0
    //  no simd       24       47        0
    fn mul(self, other: Sphere) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereAtOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        5        0
    // no simd        0       20        0
    fn mul(self, other: SphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        8        0
    //    simd4        6        6        0
    // Totals...
    // yes simd        6       14        0
    //  no simd       24       32        0
    fn mul(self, other: SphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for AntiDipoleInversionOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       44        0
    //    simd3        0        3        0
    //    simd4       21       19        0
    // Totals...
    // yes simd       52       66        0
    //  no simd      115      129        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAligningOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       45        0
    //    simd3        0        4        0
    //    simd4       14       10        0
    // Totals...
    // yes simd       44       59        0
    //  no simd       86       97        0
    fn mul(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtInfinity> for AntiDipoleInversionOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       32        0
    //    simd3        0        3        0
    //    simd4       20       18        0
    // Totals...
    // yes simd       39       53        0
    //  no simd       99      113        0
    fn mul(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       20        0
    //    simd3        0        4        0
    //    simd4       10        8        0
    // Totals...
    // yes simd       18       32        0
    //  no simd       48       64        0
    fn mul(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       17        0
    //    simd3        0        1        0
    //    simd4        8        7        0
    // Totals...
    // yes simd       20       25        0
    //  no simd       44       48        0
    fn mul(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOrthogonalOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       36        0
    //    simd3        0        4        0
    //    simd4       16       12        0
    // Totals...
    // yes simd       36       52        0
    //  no simd       84       96        0
    fn mul(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       51        0
    //    simd3        0        2        0
    //    simd4       20       18        0
    // Totals...
    // yes simd       52       71        0
    //  no simd      112      129        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddAtInfinity> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       41        0
    //    simd3        0        1        0
    //    simd4       18       17        0
    // Totals...
    // yes simd       42       59        0
    //  no simd       96      112        0
    fn mul(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddOrthogonalOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       50        0
    //    simd3        0        1        0
    //    simd4       12       11        0
    // Totals...
    // yes simd       44       62        0
    //  no simd       80       97        0
    fn mul(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Neg for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn neg(self) -> Self::Output {
        return AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from(-1.0),
            // e4, e1, e2, e3
            self.group1() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Not for AntiDipoleInversionOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiCircleOnOrigin> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotor> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOrigin> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOriginAtInfinity> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAtInfinity> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorOnOrigin> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiDipoleInversion> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        2        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        4        3        0
    //  no simd        8       12        0
    fn sub(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz() - other.group0(),
            // e415, e425, e435, e321
            other.group1().xyz().with_w(self[e321] - other[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            other.group2().xyz().with_w(self[e4] - other[e4]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            (self.group1().yzw() - other.group3().xyz()).with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionAtInfinity> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        4       12        0
    fn sub(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            other.group0().xyz().with_w(self[e321] - other[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            other.group1().with_w(self[e4]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            (self.group1().yzw() - other.group2().xyz()).with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn sub(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() - other.group0(),
            // e4, e1, e2, e3
            self.group1() - other.group1(),
        );
    }
}
impl std::ops::SubAssign<AntiDipoleInversionOnOrigin> for AntiDipoleInversionOnOrigin {
    fn sub_assign(&mut self, other: AntiDipoleInversionOnOrigin) {
        *self = AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() - other.group0(),
            // e4, e1, e2, e3
            self.group1() - other.group1(),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOrthogonalOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        4       12        0
    fn sub(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz() - other.group0().xyz(),
            // e415, e425, e435, e321
            other.group1().with_w(self[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            other.group2().xyz().with_w(self[e4] - other[e4]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            self.group1().yzw().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<AntiDipoleOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return AntiDipoleInversionOnOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0() - other.group0(), /* e4, e1, e2, e3 */ self.group1());
    }
}
impl std::ops::SubAssign<AntiDipoleOnOrigin> for AntiDipoleInversionOnOrigin {
    fn sub_assign(&mut self, other: AntiDipoleOnOrigin) {
        *self = AntiDipoleInversionOnOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0() - other.group0(), /* e4, e1, e2, e3 */ self.group1());
    }
}
impl std::ops::Sub<AntiDualNum> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiFlatOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e4, e1, e2, e3
            self.group1(),
        );
    }
}
impl std::ops::SubAssign<AntiFlatOrigin> for AntiDipoleInversionOnOrigin {
    fn sub_assign(&mut self, other: AntiFlatOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e4, e1, e2, e3
            self.group1(),
        );
    }
}
impl std::ops::Sub<AntiFlatPoint> for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        4        4        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125, e5
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
        );
    }
}
impl std::ops::Sub<AntiFlector> for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        8        8        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125, e5
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from(-1.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group1().yzwx(),
        );
    }
}
impl std::ops::Sub<AntiFlectorOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        4        0
    fn sub(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e4, e1, e2, e3
            Simd32x4::from([0.0, other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0]) + self.group1(),
        );
    }
}
impl std::ops::SubAssign<AntiFlectorOnOrigin> for AntiDipoleInversionOnOrigin {
    fn sub_assign(&mut self, other: AntiFlectorOnOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e4, e1, e2, e3
            Simd32x4::from([0.0, other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0]) + self.group1(),
        );
    }
}
impl std::ops::Sub<AntiLine> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiLineOnOrigin> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiMotor> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<AntiMotorOnOrigin> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiMysteryCircleRotor> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiMysteryDipoleInversion> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        4        4        0
    fn sub(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            other.group0().xyz().with_w(self[e321] - other[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e1, e2, e3, e5
            (self.group1().yzw() - other.group1()).with_w(0.0),
        );
    }
}
impl std::ops::Sub<AntiPlane> for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        4        7        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(other[e5]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group1().yzwx(),
        );
    }
}
impl std::ops::Sub<AntiPlaneOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        4        3        0
    fn sub(self, other: AntiPlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e4, e1, e2, e3
            Simd32x4::from([0.0, other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0]) + self.group1(),
        );
    }
}
impl std::ops::SubAssign<AntiPlaneOnOrigin> for AntiDipoleInversionOnOrigin {
    fn sub_assign(&mut self, other: AntiPlaneOnOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e4, e1, e2, e3
            Simd32x4::from([0.0, other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0]) + self.group1(),
        );
    }
}
impl std::ops::Sub<AntiScalar> for AntiDipoleInversionOnOrigin {
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
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
        );
    }
}
impl std::ops::Sub<AntiSphereOnOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: AntiSphereOnOrigin) -> Self::Output {
        return AntiDipoleInversionOnOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0(), /* e4, e1, e2, e3 */ self.group1() - other.group0().wxyz());
    }
}
impl std::ops::SubAssign<AntiSphereOnOrigin> for AntiDipoleInversionOnOrigin {
    fn sub_assign(&mut self, other: AntiSphereOnOrigin) {
        *self = AntiDipoleInversionOnOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0(), /* e4, e1, e2, e3 */ self.group1() - other.group0().wxyz());
    }
}
impl std::ops::Sub<AntiVersorEvenOnOrigin> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Circle> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        4        8        0
    fn sub(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz() - other.group0(),
            // e415, e425, e435, e321
            other.group1().xyz().with_w(self[e321] - other[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            other.group2().with_w(self[e4]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], 0.0]),
        );
    }
}
impl std::ops::Sub<CircleAligningOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        3        8        0
    fn sub(self, other: CircleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz() - other.group0(),
            // e415, e425, e435, e321
            other.group1().with_w(self[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            other.group2().with_w(self[e4]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], 0.0]),
        );
    }
}
impl std::ops::Sub<CircleAtInfinity> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        1        8        0
    fn sub(self, other: CircleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            other.group0().xyz().with_w(self[e321] - other[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            other.group1().with_w(self[e4]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], 0.0]),
        );
    }
}
impl std::ops::Sub<CircleAtOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        4        6        0
    fn sub(self, other: CircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e235, e315, e125, e5
            (other.group1() * Simd32x3::from(-1.0)).with_w(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
        );
    }
}
impl std::ops::Sub<CircleOnOrigin> for AntiDipoleInversionOnOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], 0.0]),
        );
    }
}
impl std::ops::Sub<CircleOrthogonalOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        3        0
    fn sub(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() - other.group0(),
            // e235, e315, e125, e5
            (other.group1() * Simd32x3::from(-1.0)).with_w(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
        );
    }
}
impl std::ops::Sub<CircleRotor> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        4       11        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0().xyz() - other.group0()).with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            other.group1().xyz().with_w(self[e321] - other[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            (other.group2().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        3       11        0
    fn sub(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0().xyz() - other.group0()).with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            other.group1().with_w(self[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            (other.group2().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOriginAtInfinity> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn sub(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            other.group0().with_w(self[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
        );
    }
}
impl std::ops::Sub<CircleRotorAtInfinity> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        1       11        0
    fn sub(self, other: CircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            other.group0().xyz().with_w(self[e321] - other[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
        );
    }
}
impl std::ops::Sub<CircleRotorOnOrigin> for AntiDipoleInversionOnOrigin {
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
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
        );
    }
}
impl std::ops::Sub<Dipole> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleAligningOrigin> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleAtInfinity> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleAtOrigin> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleInversion> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionAligningOrigin> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionAtInfinity> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionAtOrigin> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionOnOrigin> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            other.group1() * Simd32x4::from(-1.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionOrthogonalOrigin> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleOnOrigin> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleOrthogonalOrigin> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DualNum> for AntiDipoleInversionOnOrigin {
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
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::Sub<FlatOrigin> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<FlatPoint> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<FlatPointAtInfinity> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Flector> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<FlectorAtInfinity> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<FlectorOnOrigin> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Horizon> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<Infinity> for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(other[e5]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e1, e2, e3, e4
            self.group1().yzwx(),
        );
    }
}
impl std::ops::Sub<Line> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            other.group0().with_w(self[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            other.group1().with_w(self[e4]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], 0.0]),
        );
    }
}
impl std::ops::Sub<LineAtInfinity> for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: LineAtInfinity) -> Self::Output {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
        );
    }
}
impl std::ops::Sub<LineOnOrigin> for AntiDipoleInversionOnOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], 0.0]),
        );
    }
}
impl std::ops::Sub<Motor> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn sub(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            other.group0().xyz().with_w(self[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from(-1.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
        );
    }
}
impl std::ops::Sub<MotorAtInfinity> for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: MotorAtInfinity) -> Self::Output {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            other.group0() * Simd32x4::from(-1.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
        );
    }
}
impl std::ops::Sub<MotorOnOrigin> for AntiDipoleInversionOnOrigin {
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
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
        );
    }
}
impl std::ops::Sub<MultiVector> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd2        0        1        0
    //    simd3        1        3        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        3        9        0
    //  no simd        8       25        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            other.group0() * Simd32x2::from(-1.0),
            // e1, e2, e3, e4
            self.group1().yzwx() - other.group1(),
            // e5
            other[e5] * -1.0,
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
            other.group8() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            other.group9() * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<MysteryCircle> for AntiDipoleInversionOnOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], 0.0]),
        );
    }
}
impl std::ops::Sub<MysteryCircleRotor> for AntiDipoleInversionOnOrigin {
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
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx(),
        );
    }
}
impl std::ops::Sub<MysteryDipole> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MysteryDipoleInversion> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MysteryVersorEven> for AntiDipoleInversionOnOrigin {
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
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group1().yzwx(),
        );
    }
}
impl std::ops::Sub<MysteryVersorOdd> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullCircleAtOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        4        3        0
    fn sub(self, other: NullCircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e4, e1, e2, e3
            self.group1(),
        );
    }
}
impl std::ops::SubAssign<NullCircleAtOrigin> for AntiDipoleInversionOnOrigin {
    fn sub_assign(&mut self, other: NullCircleAtOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e4, e1, e2, e3
            self.group1(),
        );
    }
}
impl std::ops::Sub<NullDipoleAtOrigin> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullDipoleInversionAtOrigin> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullSphereAtOrigin> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullVersorEvenAtOrigin> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        4        0
    fn sub(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e4, e1, e2, e3
            Simd32x4::from([other[e4] * -1.0, 0.0, 0.0, 0.0]) + self.group1(),
        );
    }
}
impl std::ops::SubAssign<NullVersorEvenAtOrigin> for AntiDipoleInversionOnOrigin {
    fn sub_assign(&mut self, other: NullVersorEvenAtOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e4, e1, e2, e3
            Simd32x4::from([other[e4] * -1.0, 0.0, 0.0, 0.0]) + self.group1(),
        );
    }
}
impl std::ops::Sub<Origin> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e4, e1, e2, e3
            Simd32x4::from([other[e4] * -1.0, 0.0, 0.0, 0.0]) + self.group1(),
        );
    }
}
impl std::ops::SubAssign<Origin> for AntiDipoleInversionOnOrigin {
    fn sub_assign(&mut self, other: Origin) {
        use crate::elements::*;
        *self = AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e4, e1, e2, e3
            Simd32x4::from([other[e4] * -1.0, 0.0, 0.0, 0.0]) + self.group1(),
        );
    }
}
impl std::ops::Sub<Plane> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<PlaneOnOrigin> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<RoundPoint> for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        1        0
    // no simd        4        4        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(other[e5]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e1, e2, e3, e4
            self.group1().yzwx() - other.group0(),
        );
    }
}
impl std::ops::Sub<RoundPointAtOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        4        5        0
    fn sub(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(other[e5]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e1, e2, e3, e4
            self.group1().yzwx() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::Sub<Scalar> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Sphere> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<SphereAtOrigin> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<SphereOnOrigin> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            other.group0().wxyz() * Simd32x4::from(-1.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<VersorEven> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        8       12        0
    fn sub(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0().xyz() - other.group0().xyz()).with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            other.group1().xyz().with_w(self[e321] - other[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group2() * Simd32x4::from(-1.0),
            // e1, e2, e3, e4
            self.group1().yzwx() - other.group3(),
        );
    }
}
impl std::ops::Sub<VersorEvenAligningOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        0        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        7       13        0
    fn sub(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0().xyz() - other.group0().xyz()).with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            other.group1().xyz().with_w(self[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group2() * Simd32x4::from(-1.0),
            // e1, e2, e3, e4
            self.group1().yzwx() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::Sub<VersorEvenAtInfinity> for AntiDipoleInversionOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        5       15        0
    fn sub(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            other.group1().xyz().with_w(self[e321] - other[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group2() * Simd32x4::from(-1.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group1().yzwx(),
        );
    }
}
impl std::ops::Sub<VersorEvenAtOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        8        8        0
    fn sub(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from(-1.0),
            // e1, e2, e3, e4
            self.group1().yzwx() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::Sub<VersorEvenOnOrigin> for AntiDipoleInversionOnOrigin {
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
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            self.group1().yzwx() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::Sub<VersorEvenOrthogonalOrigin> for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        1        0
    // no simd        8        4        0
    fn sub(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() - other.group0(),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from(-1.0),
            // e1, e2, e3, e4
            self.group1().yzwx() - other.group2(),
        );
    }
}
impl std::ops::Sub<VersorOdd> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<VersorOddAtInfinity> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<VersorOddOrthogonalOrigin> for AntiDipoleInversionOnOrigin {
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
            self.group1().yzwx(),
            // e5
            0.0,
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
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}

impl TryFrom<AntiDipoleInversion> for AntiDipoleInversionOnOrigin {
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
        let el = anti_dipole_inversion[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[14];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversion do not fit into AntiDipoleInversionOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            anti_dipole_inversion.group0().with_w(anti_dipole_inversion[e321]),
            // e4, e1, e2, e3
            Simd32x4::from([anti_dipole_inversion[e4], anti_dipole_inversion[e1], anti_dipole_inversion[e2], anti_dipole_inversion[e3]]),
        ));
    }
}

impl TryFrom<AntiDipoleInversionAtInfinity> for AntiDipoleInversionOnOrigin {
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
        let el = anti_dipole_inversion_at_infinity[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversionAtInfinity do not fit into AntiDipoleInversionOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(anti_dipole_inversion_at_infinity[e321]),
            // e4, e1, e2, e3
            Simd32x4::from([0.0, anti_dipole_inversion_at_infinity[e1], anti_dipole_inversion_at_infinity[e2], anti_dipole_inversion_at_infinity[e3]]),
        ));
    }
}

impl TryFrom<AntiDipoleInversionOrthogonalOrigin> for AntiDipoleInversionOnOrigin {
    type Error = String;
    fn try_from(anti_dipole_inversion_orthogonal_origin: AntiDipoleInversionOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion_orthogonal_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
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
        let el = anti_dipole_inversion_orthogonal_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_orthogonal_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_orthogonal_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversionOrthogonalOrigin do not fit into AntiDipoleInversionOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            anti_dipole_inversion_orthogonal_origin.group0().xyz().with_w(0.0),
            // e4, e1, e2, e3
            Simd32x4::from([anti_dipole_inversion_orthogonal_origin[e4], 0.0, 0.0, 0.0]),
        ));
    }
}

impl TryFrom<AntiFlatPoint> for AntiDipoleInversionOnOrigin {
    type Error = String;
    fn try_from(anti_flat_point: AntiFlatPoint) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_flat_point[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flat_point[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flat_point[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiFlatPoint do not fit into AntiDipoleInversionOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(anti_flat_point[e321]),
            // e4, e1, e2, e3
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiFlector> for AntiDipoleInversionOnOrigin {
    type Error = String;
    fn try_from(anti_flector: AntiFlector) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_flector[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiFlector do not fit into AntiDipoleInversionOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(anti_flector[e321]),
            // e4, e1, e2, e3
            Simd32x4::from([0.0, anti_flector[e1], anti_flector[e2], anti_flector[e3]]),
        ));
    }
}

impl TryFrom<AntiMysteryDipoleInversion> for AntiDipoleInversionOnOrigin {
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
            let mut error = "Elements from AntiMysteryDipoleInversion do not fit into AntiDipoleInversionOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(anti_mystery_dipole_inversion[e321]),
            // e4, e1, e2, e3
            Simd32x4::from([0.0, anti_mystery_dipole_inversion[e1], anti_mystery_dipole_inversion[e2], anti_mystery_dipole_inversion[e3]]),
        ));
    }
}

impl TryFrom<AntiPlane> for AntiDipoleInversionOnOrigin {
    type Error = String;
    fn try_from(anti_plane: AntiPlane) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_plane[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiPlane do not fit into AntiDipoleInversionOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e4, e1, e2, e3
            Simd32x4::from([0.0, anti_plane[e1], anti_plane[e2], anti_plane[e3]]),
        ));
    }
}

impl TryFrom<Circle> for AntiDipoleInversionOnOrigin {
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
        let el = circle[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Circle do not fit into AntiDipoleInversionOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            circle.group0().with_w(circle[e321]),
            // e4, e1, e2, e3
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleAligningOrigin> for AntiDipoleInversionOnOrigin {
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
        let el = circle_aligning_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_aligning_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_aligning_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleAligningOrigin do not fit into AntiDipoleInversionOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            circle_aligning_origin.group0().with_w(0.0),
            // e4, e1, e2, e3
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleAtInfinity> for AntiDipoleInversionOnOrigin {
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
        let el = circle_at_infinity[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_at_infinity[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleAtInfinity do not fit into AntiDipoleInversionOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(circle_at_infinity[e321]),
            // e4, e1, e2, e3
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleAtOrigin> for AntiDipoleInversionOnOrigin {
    type Error = String;
    fn try_from(circle_at_origin: CircleAtOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_at_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_at_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_at_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleAtOrigin do not fit into AntiDipoleInversionOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            circle_at_origin.group0().with_w(0.0),
            // e4, e1, e2, e3
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleOnOrigin> for AntiDipoleInversionOnOrigin {
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
            let mut error = "Elements from CircleOnOrigin do not fit into AntiDipoleInversionOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            circle_on_origin.group0().with_w(0.0),
            // e4, e1, e2, e3
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleOrthogonalOrigin> for AntiDipoleInversionOnOrigin {
    type Error = String;
    fn try_from(circle_orthogonal_origin: CircleOrthogonalOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_orthogonal_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_orthogonal_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_orthogonal_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleOrthogonalOrigin do not fit into AntiDipoleInversionOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            circle_orthogonal_origin.group0(),
            // e4, e1, e2, e3
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleRotor> for AntiDipoleInversionOnOrigin {
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
        let el = circle_rotor[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
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
            let mut error = "Elements from CircleRotor do not fit into AntiDipoleInversionOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            circle_rotor.group0().with_w(circle_rotor[e321]),
            // e4, e1, e2, e3
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleRotorAligningOrigin> for AntiDipoleInversionOnOrigin {
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
        let el = circle_rotor_aligning_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
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
            let mut error = "Elements from CircleRotorAligningOrigin do not fit into AntiDipoleInversionOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            circle_rotor_aligning_origin.group0().with_w(0.0),
            // e4, e1, e2, e3
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleRotorAtInfinity> for AntiDipoleInversionOnOrigin {
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
        let el = circle_rotor_at_infinity[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_at_infinity[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
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
            let mut error = "Elements from CircleRotorAtInfinity do not fit into AntiDipoleInversionOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(circle_rotor_at_infinity[e321]),
            // e4, e1, e2, e3
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleRotorOnOrigin> for AntiDipoleInversionOnOrigin {
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
            let mut error = "Elements from CircleRotorOnOrigin do not fit into AntiDipoleInversionOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            circle_rotor_on_origin.group0().xyz().with_w(0.0),
            // e4, e1, e2, e3
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<DualNum> for AntiDipoleInversionOnOrigin {
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
            let mut error = "Elements from DualNum do not fit into AntiDipoleInversionOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e4, e1, e2, e3
            Simd32x4::from([dual_num[e4], 0.0, 0.0, 0.0]),
        ));
    }
}

impl TryFrom<MultiVector> for AntiDipoleInversionOnOrigin {
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
        let el = multi_vector[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
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
            let mut error = "Elements from MultiVector do not fit into AntiDipoleInversionOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            multi_vector.group7().with_w(multi_vector[e321]),
            // e4, e1, e2, e3
            multi_vector.group1().wxyz(),
        ));
    }
}

impl TryFrom<MysteryCircle> for AntiDipoleInversionOnOrigin {
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
            let mut error = "Elements from MysteryCircle do not fit into AntiDipoleInversionOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(mystery_circle[e321]),
            // e4, e1, e2, e3
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<MysteryCircleRotor> for AntiDipoleInversionOnOrigin {
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
            let mut error = "Elements from MysteryCircleRotor do not fit into AntiDipoleInversionOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(mystery_circle_rotor[e321]),
            // e4, e1, e2, e3
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<MysteryVersorEven> for AntiDipoleInversionOnOrigin {
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
            let mut error = "Elements from MysteryVersorEven do not fit into AntiDipoleInversionOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(mystery_versor_even[e321]),
            // e4, e1, e2, e3
            Simd32x4::from([0.0, mystery_versor_even[e1], mystery_versor_even[e2], mystery_versor_even[e3]]),
        ));
    }
}

impl TryFrom<RoundPoint> for AntiDipoleInversionOnOrigin {
    type Error = String;
    fn try_from(round_point: RoundPoint) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = round_point[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from RoundPoint do not fit into AntiDipoleInversionOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e4, e1, e2, e3
            round_point.group0().wxyz(),
        ));
    }
}

impl TryFrom<RoundPointAtOrigin> for AntiDipoleInversionOnOrigin {
    type Error = String;
    fn try_from(round_point_at_origin: RoundPointAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = round_point_at_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from RoundPointAtOrigin do not fit into AntiDipoleInversionOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e4, e1, e2, e3
            Simd32x4::from([round_point_at_origin[e4], 0.0, 0.0, 0.0]),
        ));
    }
}

impl TryFrom<VersorEven> for AntiDipoleInversionOnOrigin {
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
        let el = versor_even[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEven do not fit into AntiDipoleInversionOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([versor_even[e423], versor_even[e431], versor_even[e412], versor_even[e321]]),
            // e4, e1, e2, e3
            versor_even.group3().wxyz(),
        ));
    }
}

impl TryFrom<VersorEvenAligningOrigin> for AntiDipoleInversionOnOrigin {
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
        let el = versor_even_aligning_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAligningOrigin do not fit into AntiDipoleInversionOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            versor_even_aligning_origin.group0().xyz().with_w(0.0),
            // e4, e1, e2, e3
            Simd32x4::from([versor_even_aligning_origin[e4], 0.0, 0.0, 0.0]),
        ));
    }
}

impl TryFrom<VersorEvenAtInfinity> for AntiDipoleInversionOnOrigin {
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
        let el = versor_even_at_infinity[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAtInfinity do not fit into AntiDipoleInversionOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(versor_even_at_infinity[e321]),
            // e4, e1, e2, e3
            Simd32x4::from([0.0, versor_even_at_infinity[e1], versor_even_at_infinity[e2], versor_even_at_infinity[e3]]),
        ));
    }
}

impl TryFrom<VersorEvenAtOrigin> for AntiDipoleInversionOnOrigin {
    type Error = String;
    fn try_from(versor_even_at_origin: VersorEvenAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_at_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAtOrigin do not fit into AntiDipoleInversionOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            versor_even_at_origin.group0().xyz().with_w(0.0),
            // e4, e1, e2, e3
            Simd32x4::from([versor_even_at_origin[e4], 0.0, 0.0, 0.0]),
        ));
    }
}

impl TryFrom<VersorEvenOnOrigin> for AntiDipoleInversionOnOrigin {
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
            let mut error = "Elements from VersorEvenOnOrigin do not fit into AntiDipoleInversionOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            versor_even_on_origin.group0().xyz().with_w(0.0),
            // e4, e1, e2, e3
            Simd32x4::from([versor_even_on_origin[e4], 0.0, 0.0, 0.0]),
        ));
    }
}

impl TryFrom<VersorEvenOrthogonalOrigin> for AntiDipoleInversionOnOrigin {
    type Error = String;
    fn try_from(versor_even_orthogonal_origin: VersorEvenOrthogonalOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_orthogonal_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenOrthogonalOrigin do not fit into AntiDipoleInversionOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            versor_even_orthogonal_origin.group0(),
            // e4, e1, e2, e3
            versor_even_orthogonal_origin.group2().wxyz(),
        ));
    }
}
