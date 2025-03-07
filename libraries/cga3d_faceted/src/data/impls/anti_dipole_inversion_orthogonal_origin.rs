use crate::traits::GeometricProduct;
use crate::traits::RightDual;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 330
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         1       2       0
//  Average:         8      13       0
//  Maximum:       122     160       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       4       0
//  Average:        19      24       0
//  Maximum:       320     357       0
impl std::ops::Add<AntiCircleOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiCircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<AntiCircleRotor> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(other[e45]),
            // e15, e25, e35
            other.group2().xyz(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<AntiCircleRotorAligningOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(0.0),
            // e15, e25, e35
            other.group2().xyz(),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<AntiCircleRotorAligningOriginAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1().xyz(),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<AntiCircleRotorAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            other.group1().xyz(),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<AntiCircleRotorOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().xyz().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<AntiDipoleInversion> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       14        0        0
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() + self.group0().xyz(),
            // e415, e425, e435, e321
            (self.group1() + other.group1().xyz()).with_w(other[e321]),
            // e235, e315, e125, e4
            other.group2() + self.group2(),
            // e1, e2, e3, e5
            other.group3() + Simd32x3::from(0.0).with_w(self[e5]),
        );
    }
}
impl std::ops::Add<AntiDipoleInversionAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn add(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            (self.group1() + other.group0().xyz()).with_w(other[e321]),
            // e235, e315, e125, e4
            (other.group1() + self.group2().xyz()).with_w(self[e4]),
            // e1, e2, e3, e5
            other.group2() + Simd32x3::from(0.0).with_w(self[e5]),
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0().xyz() + self.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(other[e321]),
            // e235, e315, e125, e4
            self.group2().xyz().with_w(other[e4] + self[e4]),
            // e1, e2, e3, e5
            Simd32x4::from([other[e1], other[e2], other[e3], self[e5]]),
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOrthogonalOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       11        0        0
    fn add(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            other.group0() + self.group0(),
            // e415, e425, e435
            other.group1() + self.group1(),
            // e235, e315, e125, e4
            other.group2() + self.group2(),
        );
    }
}
impl std::ops::AddAssign<AntiDipoleInversionOrthogonalOrigin> for AntiDipoleInversionOrthogonalOrigin {
    fn add_assign(&mut self, other: AntiDipoleInversionOrthogonalOrigin) {
        *self = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            other.group0() + self.group0(),
            // e415, e425, e435
            other.group1() + self.group1(),
            // e235, e315, e125, e4
            other.group2() + self.group2(),
        );
    }
}
impl std::ops::Add<AntiDipoleOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: AntiDipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz() + other.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(other[e321]),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(self[e5]),
        );
    }
}
impl std::ops::Add<AntiDualNum> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<AntiFlatOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    fn add(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(other[e321]),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(self[e5]),
        );
    }
}
impl std::ops::Add<AntiFlatPoint> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(other[e321]),
            // e235, e315, e125, e4
            self.group2() + other.group0().xyz().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(self[e5]),
        );
    }
}
impl std::ops::Add<AntiFlector> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(other[e321]),
            // e235, e315, e125, e4
            self.group2() + other.group0().xyz().with_w(0.0),
            // e1, e2, e3, e5
            other.group1().xyz().with_w(self[e5] + other[e5]),
        );
    }
}
impl std::ops::Add<AntiFlectorOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    fn add(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(other[e321]),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            Simd32x4::from([other[e1], other[e2], other[e3], self[e5]]),
        );
    }
}
impl std::ops::Add<AntiLine> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<AntiLineOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiLineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<AntiMotor> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1().xyz(),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<AntiMotorOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<AntiMysteryCircleRotor> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<AntiMysteryDipoleInversion> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            (self.group1() + other.group0().xyz()).with_w(other[e321]),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            other.group1().with_w(self[e5]),
        );
    }
}
impl std::ops::Add<AntiPlane> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            other.group0().xyz().with_w(self[e5] + other[e5]),
        );
    }
}
impl std::ops::Add<AntiPlaneOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    fn add(self, other: AntiPlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            other.group0().with_w(self[e5]),
        );
    }
}
impl std::ops::Add<AntiScalar> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]),
            // e415, e425, e435, e4
            self.group1().with_w(self[e4]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl std::ops::Add<AntiSphereOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: AntiSphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
            // e235, e315, e125, e4
            self.group2() + Simd32x3::from(0.0).with_w(other[e4]),
            // e1, e2, e3, e5
            other.group0().xyz().with_w(self[e5]),
        );
    }
}
impl std::ops::Add<AntiVersorEvenOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().xyz().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<Circle> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        3        0        0
    // no simd        9        0        0
    fn add(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() + self.group0().xyz(),
            // e415, e425, e435, e321
            (self.group1() + other.group1().xyz()).with_w(other[e321]),
            // e235, e315, e125, e4
            (other.group2() + self.group2().xyz()).with_w(self[e4]),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(self[e5]),
        );
    }
}
impl std::ops::Add<CircleAligningOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        3        0        0
    // no simd        9        0        0
    fn add(self, other: CircleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            (other.group0() + self.group0().xyz()).with_w(self[e5]),
            // e415, e425, e435
            self.group1() + other.group1(),
            // e235, e315, e125, e4
            (other.group2() + self.group2().xyz()).with_w(self[e4]),
        );
    }
}
impl std::ops::AddAssign<CircleAligningOrigin> for AntiDipoleInversionOrthogonalOrigin {
    fn add_assign(&mut self, other: CircleAligningOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            (other.group0() + self.group0().xyz()).with_w(self[e5]),
            // e415, e425, e435
            self.group1() + other.group1(),
            // e235, e315, e125, e4
            (other.group2() + self.group2().xyz()).with_w(self[e4]),
        );
    }
}
impl std::ops::Add<CircleAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: CircleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            (self.group1() + other.group0().xyz()).with_w(other[e321]),
            // e235, e315, e125, e4
            (other.group1() + self.group2().xyz()).with_w(self[e4]),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(self[e5]),
        );
    }
}
impl std::ops::Add<CircleAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: CircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            (other.group0() + self.group0().xyz()).with_w(self[e5]),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            (other.group1() + self.group2().xyz()).with_w(self[e4]),
        );
    }
}
impl std::ops::AddAssign<CircleAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    fn add_assign(&mut self, other: CircleAtOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            (other.group0() + self.group0().xyz()).with_w(self[e5]),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            (other.group1() + self.group2().xyz()).with_w(self[e4]),
        );
    }
}
impl std::ops::Add<CircleOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: CircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            (other.group0() + self.group0().xyz()).with_w(self[e5]),
            // e415, e425, e435
            self.group1() + other.group1(),
            // e235, e315, e125, e4
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<CircleOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    fn add_assign(&mut self, other: CircleOnOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            (other.group0() + self.group0().xyz()).with_w(self[e5]),
            // e415, e425, e435
            self.group1() + other.group1(),
            // e235, e315, e125, e4
            self.group2(),
        );
    }
}
impl std::ops::Add<CircleOrthogonalOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: CircleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz() + other.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(other[e321]),
            // e235, e315, e125, e4
            (other.group1() + self.group2().xyz()).with_w(self[e4]),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(self[e5]),
        );
    }
}
impl std::ops::Add<CircleRotor> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        3        0        0
    // no simd        9        0        0
    fn add(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() + self.group0().xyz()).with_w(other[e12345]),
            // e415, e425, e435, e321
            (self.group1() + other.group1().xyz()).with_w(other[e321]),
            // e235, e315, e125, e5
            (self.group2().xyz() + other.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
        );
    }
}
impl std::ops::Add<CircleRotorAligningOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        3        0        0
    // no simd        9        0        0
    fn add(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            (other.group0() + self.group0().xyz()).with_w(other[e12345]),
            // e415, e425, e435, e4
            (self.group1() + other.group1()).with_w(self[e4]),
            // e235, e315, e125, e5
            (self.group2().xyz() + other.group2().xyz()).with_w(self[e5]),
        );
    }
}
impl std::ops::Add<CircleRotorAligningOriginAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]),
            // e415, e425, e435, e4
            (self.group1() + other.group0()).with_w(self[e4]),
            // e235, e315, e125, e5
            (self.group2().xyz() + other.group1().xyz()).with_w(self[e5]),
        );
    }
}
impl std::ops::Add<CircleRotorAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: CircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]),
            // e415, e425, e435, e321
            (self.group1() + other.group0().xyz()).with_w(other[e321]),
            // e235, e315, e125, e5
            (self.group2().xyz() + other.group1().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
        );
    }
}
impl std::ops::Add<CircleRotorOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: CircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            (self.group0().xyz() + other.group0().xyz()).with_w(other[e12345]),
            // e415, e425, e435, e4
            (self.group1() + other.group1()).with_w(self[e4]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl std::ops::Add<Dipole> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(other[e45]),
            // e15, e25, e35
            other.group2(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<DipoleAligningOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<DipoleAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<DipoleAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(0.0),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<DipoleInversion> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(other[e45]),
            // e15, e25, e35
            other.group2().xyz(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<DipoleInversionAligningOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            other.group1().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<DipoleInversionAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<DipoleInversionAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().xyz().with_w(0.0),
            // e15, e25, e35
            other.group1().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<DipoleInversionOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<DipoleInversionOrthogonalOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().xyz().with_w(0.0),
            // e15, e25, e35
            other.group2().xyz(),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<DipoleOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<DipoleOrthogonalOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(0.0),
            // e15, e25, e35
            other.group2(),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<DualNum> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]),
            // e415, e425, e435, e4
            self.group1().with_w(self[e4] + other[e4]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl std::ops::Add<FlatOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<FlatPoint> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            other.group0().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<FlatPointAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: FlatPointAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group0(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<Flector> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            other.group0().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<FlectorAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group0().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<FlectorOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<Horizon> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<Infinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0() + Simd32x3::from(0.0).with_w(other[e5]),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<Infinity> for AntiDipoleInversionOrthogonalOrigin {
    fn add_assign(&mut self, other: Infinity) {
        use crate::elements::*;
        *self = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0() + Simd32x3::from(0.0).with_w(other[e5]),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            self.group2(),
        );
    }
}
impl std::ops::Add<Line> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0(),
            // e415, e425, e435
            self.group1() + other.group0(),
            // e235, e315, e125, e4
            (other.group1() + self.group2().xyz()).with_w(self[e4]),
        );
    }
}
impl std::ops::AddAssign<Line> for AntiDipoleInversionOrthogonalOrigin {
    fn add_assign(&mut self, other: Line) {
        use crate::elements::*;
        *self = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0(),
            // e415, e425, e435
            self.group1() + other.group0(),
            // e235, e315, e125, e4
            (other.group1() + self.group2().xyz()).with_w(self[e4]),
        );
    }
}
impl std::ops::Add<LineAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: LineAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0(),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            (other.group0() + self.group2().xyz()).with_w(self[e4]),
        );
    }
}
impl std::ops::AddAssign<LineAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    fn add_assign(&mut self, other: LineAtInfinity) {
        use crate::elements::*;
        *self = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0(),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            (other.group0() + self.group2().xyz()).with_w(self[e4]),
        );
    }
}
impl std::ops::Add<LineOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: LineOnOrigin) -> Self::Output {
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0(),
            // e415, e425, e435
            self.group1() + other.group0(),
            // e235, e315, e125, e4
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<LineOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    fn add_assign(&mut self, other: LineOnOrigin) {
        *self = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0(),
            // e415, e425, e435
            self.group1() + other.group0(),
            // e235, e315, e125, e4
            self.group2(),
        );
    }
}
impl std::ops::Add<Motor> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]),
            // e415, e425, e435, e4
            (self.group1() + other.group0().xyz()).with_w(self[e4]),
            // e235, e315, e125, e5
            other.group1() + self.group2().xyz().with_w(self[e5]),
        );
    }
}
impl std::ops::Add<MotorAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: MotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0() + Simd32x3::from(0.0).with_w(other[e5]),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            self.group2() + other.group0().xyz().with_w(0.0),
        );
    }
}
impl std::ops::AddAssign<MotorAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    fn add_assign(&mut self, other: MotorAtInfinity) {
        use crate::elements::*;
        *self = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0() + Simd32x3::from(0.0).with_w(other[e5]),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            self.group2() + other.group0().xyz().with_w(0.0),
        );
    }
}
impl std::ops::Add<MotorOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: MotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]),
            // e415, e425, e435, e4
            (self.group1() + other.group0().xyz()).with_w(self[e4]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl std::ops::Add<MultiVector> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        3        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       11        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            other.group0(),
            // e1, e2, e3, e4
            other.group1().xyz().with_w(self[e4] + other[e4]),
            // e5
            self[e5] + other[e5],
            // e41, e42, e43, e45
            other.group3(),
            // e15, e25, e35
            other.group4(),
            // e23, e31, e12
            other.group5(),
            // e415, e425, e435, e321
            (self.group1() + other.group6().xyz()).with_w(other[e321]),
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
impl std::ops::Add<MysteryCircle> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: MysteryCircle) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            (self.group1() + other.group0().xyz()).with_w(other[e321]),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(self[e5]),
        );
    }
}
impl std::ops::Add<MysteryCircleRotor> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]),
            // e415, e425, e435, e321
            (self.group1() + other.group0().xyz()).with_w(other[e321]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
        );
    }
}
impl std::ops::Add<MysteryDipole> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<MysteryDipoleInversion> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<MysteryVersorEven> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: MysteryVersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]),
            // e415, e425, e435, e321
            (self.group1() + other.group1().xyz()).with_w(other[e321]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], self[e4]]),
        );
    }
}
impl std::ops::Add<MysteryVersorOdd> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<NullCircleAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: NullCircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            (other.group0() + self.group0().xyz()).with_w(self[e5]),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<NullCircleAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    fn add_assign(&mut self, other: NullCircleAtOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            (other.group0() + self.group0().xyz()).with_w(self[e5]),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            self.group2(),
        );
    }
}
impl std::ops::Add<NullDipoleAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: NullDipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<NullDipoleInversionAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().xyz().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<NullSphereAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<NullVersorEvenAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0() + other.group0().xyz().with_w(0.0),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            self.group2() + Simd32x3::from(0.0).with_w(other[e4]),
        );
    }
}
impl std::ops::AddAssign<NullVersorEvenAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    fn add_assign(&mut self, other: NullVersorEvenAtOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0() + other.group0().xyz().with_w(0.0),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            self.group2() + Simd32x3::from(0.0).with_w(other[e4]),
        );
    }
}
impl std::ops::Add<Origin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0(),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            self.group2() + Simd32x3::from(0.0).with_w(other[e4]),
        );
    }
}
impl std::ops::AddAssign<Origin> for AntiDipoleInversionOrthogonalOrigin {
    fn add_assign(&mut self, other: Origin) {
        use crate::elements::*;
        *self = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0(),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            self.group2() + Simd32x3::from(0.0).with_w(other[e4]),
        );
    }
}
impl std::ops::Add<Plane> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<PlaneOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<RoundPoint> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
            // e235, e315, e125, e4
            self.group2() + Simd32x3::from(0.0).with_w(other[e4]),
            // e1, e2, e3, e5
            other.group0().xyz().with_w(self[e5] + other[e5]),
        );
    }
}
impl std::ops::Add<RoundPointAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0().xyz().with_w(self[e5] + other[e5]),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            self.group2().xyz().with_w(self[e4] + other[e4]),
        );
    }
}
impl std::ops::AddAssign<RoundPointAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    fn add_assign(&mut self, other: RoundPointAtOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0().xyz().with_w(self[e5] + other[e5]),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            self.group2().xyz().with_w(self[e4] + other[e4]),
        );
    }
}
impl std::ops::Add<Scalar> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<Sphere> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<SphereAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<SphereOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<VersorEven> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       11        0        0
    fn add(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0().xyz() + other.group0().xyz()).with_w(other[e12345]),
            // e415, e425, e435, e321
            (self.group1() + other.group1().xyz()).with_w(other[e321]),
            // e235, e315, e125, e5
            other.group2() + self.group2().xyz().with_w(self[e5]),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(self[e4] + other[e4]),
        );
    }
}
impl std::ops::Add<VersorEvenAligningOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       11        0        0
    fn add(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            (self.group0().xyz() + other.group0().xyz()).with_w(other[e12345]),
            // e415, e425, e435, e4
            other.group1() + self.group1().with_w(self[e4]),
            // e235, e315, e125, e5
            other.group2() + self.group2().xyz().with_w(self[e5]),
        );
    }
}
impl std::ops::Add<VersorEvenAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]),
            // e415, e425, e435, e321
            (self.group1() + other.group1().xyz()).with_w(other[e321]),
            // e235, e315, e125, e5
            other.group2() + self.group2().xyz().with_w(self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], self[e4]]),
        );
    }
}
impl std::ops::Add<VersorEvenAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0() + other.group0().xyz().with_w(other[e5]),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            self.group2() + other.group1().xyz().with_w(other[e4]),
        );
    }
}
impl std::ops::AddAssign<VersorEvenAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    fn add_assign(&mut self, other: VersorEvenAtOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0() + other.group0().xyz().with_w(other[e5]),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            self.group2() + other.group1().xyz().with_w(other[e4]),
        );
    }
}
impl std::ops::Add<VersorEvenOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: VersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            (self.group0().xyz() + other.group0().xyz()).with_w(other[e12345]),
            // e415, e425, e435, e4
            other.group1() + self.group1().with_w(self[e4]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl std::ops::Add<VersorEvenOrthogonalOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        8        0        0
    fn add(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz() + other.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(other[e321]),
            // e235, e315, e125, e4
            self.group2() + other.group1().xyz().with_w(other[e4]),
            // e1, e2, e3, e5
            other.group2().xyz().with_w(self[e5] + other[e5]),
        );
    }
}
impl std::ops::Add<VersorOdd> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], other[e45]]),
            // e15, e25, e35
            other.group2().xyz(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<VersorOddAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            other.group0().yzw(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Add<VersorOddOrthogonalOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().xyz().with_w(0.0),
            // e15, e25, e35
            other.group2().xyz(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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

impl From<CircleAligningOrigin> for AntiDipoleInversionOrthogonalOrigin {
    fn from(from_circle_aligning_origin: CircleAligningOrigin) -> Self {
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            from_circle_aligning_origin.group0().with_w(0.0),
            // e415, e425, e435
            from_circle_aligning_origin.group1(),
            // e235, e315, e125, e4
            from_circle_aligning_origin.group2().with_w(0.0),
        );
    }
}

impl From<CircleAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    fn from(from_circle_at_origin: CircleAtOrigin) -> Self {
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            from_circle_at_origin.group0().with_w(0.0),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e4
            from_circle_at_origin.group1().with_w(0.0),
        );
    }
}

impl From<CircleOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    fn from(from_circle_on_origin: CircleOnOrigin) -> Self {
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            from_circle_on_origin.group0().with_w(0.0),
            // e415, e425, e435
            from_circle_on_origin.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<Infinity> for AntiDipoleInversionOrthogonalOrigin {
    fn from(from_infinity: Infinity) -> Self {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x3::from(0.0).with_w(from_infinity[e5]),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<Line> for AntiDipoleInversionOrthogonalOrigin {
    fn from(from_line: Line) -> Self {
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from(0.0),
            // e415, e425, e435
            from_line.group0(),
            // e235, e315, e125, e4
            from_line.group1().with_w(0.0),
        );
    }
}

impl From<LineAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    fn from(from_line_at_infinity: LineAtInfinity) -> Self {
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from(0.0),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e4
            from_line_at_infinity.group0().with_w(0.0),
        );
    }
}

impl From<LineOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    fn from(from_line_on_origin: LineOnOrigin) -> Self {
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from(0.0),
            // e415, e425, e435
            from_line_on_origin.group0(),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<MotorAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    fn from(from_motor_at_infinity: MotorAtInfinity) -> Self {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x3::from(0.0).with_w(from_motor_at_infinity[e5]),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e4
            from_motor_at_infinity.group0().xyz().with_w(0.0),
        );
    }
}

impl From<NullCircleAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    fn from(from_null_circle_at_origin: NullCircleAtOrigin) -> Self {
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            from_null_circle_at_origin.group0().with_w(0.0),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<NullVersorEvenAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    fn from(from_null_versor_even_at_origin: NullVersorEvenAtOrigin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            from_null_versor_even_at_origin.group0().xyz().with_w(0.0),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(from_null_versor_even_at_origin[e4]),
        );
    }
}

impl From<Origin> for AntiDipoleInversionOrthogonalOrigin {
    fn from(from_origin: Origin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from(0.0),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(from_origin[e4]),
        );
    }
}

impl From<RoundPointAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    fn from(from_round_point_at_origin: RoundPointAtOrigin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x3::from(0.0).with_w(from_round_point_at_origin[e5]),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(from_round_point_at_origin[e4]),
        );
    }
}

impl From<VersorEvenAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    fn from(from_versor_even_at_origin: VersorEvenAtOrigin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from([
                from_versor_even_at_origin[e423],
                from_versor_even_at_origin[e431],
                from_versor_even_at_origin[e412],
                from_versor_even_at_origin[e5],
            ]),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e4
            Simd32x4::from([
                from_versor_even_at_origin[e235],
                from_versor_even_at_origin[e315],
                from_versor_even_at_origin[e125],
                from_versor_even_at_origin[e4],
            ]),
        );
    }
}
impl std::ops::Mul<AntiCircleOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       37        0
    //    simd3        0        3        0
    //    simd4        8        5        0
    // Totals...
    // yes simd       27       45        0
    //  no simd       51       66        0
    fn mul(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       57        0
    //    simd3        0        8        0
    //    simd4       18       10        0
    // Totals...
    // yes simd       52       75        0
    //  no simd      106      121        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       57        0
    //    simd3        0        7        0
    //    simd4       15        8        0
    // Totals...
    // yes simd       50       72        0
    //  no simd       95      110        0
    fn mul(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOriginAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       49        0
    //    simd3        0        3        0
    //    simd4        8        5        0
    // Totals...
    // yes simd       37       57        0
    //  no simd       61       78        0
    fn mul(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       52        0
    //    simd3        0        6        0
    //    simd4       11        5        0
    // Totals...
    // yes simd       42       63        0
    //  no simd       75       90        0
    fn mul(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       41        0
    //    simd3        0        4        0
    //    simd4       10        6        0
    // Totals...
    // yes simd       35       51        0
    //  no simd       65       77        0
    fn mul(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       58        0
    //    simd3        0       13        0
    //    simd4       30       17        0
    // Totals...
    // yes simd       62       88        0
    //  no simd      152      165        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       44        0
    //    simd3        0        8        0
    //    simd4       22       14        0
    // Totals...
    // yes simd       39       66        0
    //  no simd      105      124        0
    fn mul(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       29        0
    //    simd3        0        5        0
    //    simd4       16       11        0
    // Totals...
    // yes simd       32       45        0
    //  no simd       80       88        0
    fn mul(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOrthogonalOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       37       61        0
    //    simd3        0        8        0
    //    simd4       17        9        0
    // Totals...
    // yes simd       54       78        0
    //  no simd      105      121        0
    fn mul(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       21        0
    //    simd3        3        6        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       13       30        0
    //  no simd       28       51        0
    fn mul(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        2        4        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2       10        0
    //  no simd        6       27        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       19        0
    fn mul(self, other: AntiFlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd3        2        4        0
    //    simd4        5        6        0
    // Totals...
    // yes simd        9       19        0
    //  no simd       28       45        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       23        0
    //    simd3        0        4        0
    //    simd4       17       14        0
    // Totals...
    // yes simd       25       41        0
    //  no simd       76       91        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlectorOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       14       20        0
    //  no simd       35       44        0
    fn mul(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       47        0
    //    simd3        0        4        0
    //    simd4        6        2        0
    // Totals...
    // yes simd       32       53        0
    //  no simd       50       67        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLineOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       23        0
    //    simd3        0        2        0
    //    simd4        3        1        0
    // Totals...
    // yes simd       12       26        0
    //  no simd       21       33        0
    fn mul(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       55        0
    //    simd3        0        6        0
    //    simd4       10        4        0
    // Totals...
    // yes simd       42       65        0
    //  no simd       72       89        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotorOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       17       29        0
    //  no simd       32       44        0
    fn mul(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryCircleRotor> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       27        0
    //    simd3        0        5        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       24       36        0
    //  no simd       42       58        0
    fn mul(self, other: AntiMysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryDipoleInversion> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       33        0
    //    simd3        0        4        0
    //    simd4       12        8        0
    // Totals...
    // yes simd       29       45        0
    //  no simd       65       77        0
    fn mul(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       16        0
    //    simd3        2        5        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       10       26        0
    //  no simd       29       51        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlaneOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       23        0
    //    simd3        0        2        0
    //    simd4        4        2        0
    // Totals...
    // yes simd        9       27        0
    //  no simd       21       37        0
    fn mul(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for AntiDipoleInversionOrthogonalOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       22        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiSphereOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       19        0
    //    simd3        3        6        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       13       28        0
    //  no simd       28       49        0
    fn mul(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorEvenOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       37        0
    //    simd3        0        1        0
    //    simd4       13       12        0
    // Totals...
    // yes simd       37       50        0
    //  no simd       76       88        0
    fn mul(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       54        0
    //    simd3        0        8        0
    //    simd4       16        8        0
    // Totals...
    // yes simd       47       70        0
    //  no simd       95      110        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAligningOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       55        0
    //    simd3        0        8        0
    //    simd4       13        5        0
    // Totals...
    // yes simd       45       68        0
    //  no simd       84       99        0
    fn mul(self, other: CircleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       37        0
    //    simd3        0        4        0
    //    simd4       11        7        0
    // Totals...
    // yes simd       32       48        0
    //  no simd       65       77        0
    fn mul(self, other: CircleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       39        0
    //    simd3        0        4        0
    //    simd4       10        6        0
    // Totals...
    // yes simd       21       49        0
    //  no simd       51       75        0
    fn mul(self, other: CircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       46        0
    //    simd3        0        4        0
    //    simd4        6        2        0
    // Totals...
    // yes simd       32       52        0
    //  no simd       50       66        0
    fn mul(self, other: CircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOrthogonalOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       42        0
    //    simd3        0        7        0
    //    simd4       12        5        0
    // Totals...
    // yes simd       26       54        0
    //  no simd       62       83        0
    fn mul(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       59        0
    //    simd3        0        7        0
    //    simd4       18       11        0
    // Totals...
    // yes simd       52       77        0
    //  no simd      106      124        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       57        0
    //    simd3        0        7        0
    //    simd4       15        8        0
    // Totals...
    // yes simd       50       72        0
    //  no simd       95      110        0
    fn mul(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOriginAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       40        0
    //    simd3        0        3        0
    //    simd4       10        7        0
    // Totals...
    // yes simd       35       50        0
    //  no simd       65       77        0
    fn mul(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       42        0
    //    simd3        0        2        0
    //    simd4       12       10        0
    // Totals...
    // yes simd       40       54        0
    //  no simd       76       88        0
    fn mul(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       53        0
    //    simd3        0        5        0
    //    simd4        8        3        0
    // Totals...
    // yes simd       37       61        0
    //  no simd       61       80        0
    fn mul(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       56        0
    //    simd3        0       10        0
    //    simd4       16        6        0
    // Totals...
    // yes simd       47       72        0
    //  no simd       95      110        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAligningOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       35        0
    //    simd3        0        5        0
    //    simd4       12        7        0
    // Totals...
    // yes simd       26       47        0
    //  no simd       62       78        0
    fn mul(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       48        0
    //    simd3        0        5        0
    //    simd4        9        4        0
    // Totals...
    // yes simd       34       57        0
    //  no simd       61       79        0
    fn mul(self, other: DipoleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       31        0
    //    simd3        0        4        0
    //    simd4       10        6        0
    // Totals...
    // yes simd       21       41        0
    //  no simd       51       67        0
    fn mul(self, other: DipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       56        0
    //    simd3        0       10        0
    //    simd4       30       20        0
    // Totals...
    // yes simd       62       86        0
    //  no simd      152      166        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAligningOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       34        0
    //    simd3        0        6        0
    //    simd4       25       20        0
    // Totals...
    // yes simd       45       60        0
    //  no simd      120      132        0
    fn mul(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       47        0
    //    simd3        0        9        0
    //    simd4       21       12        0
    // Totals...
    // yes simd       45       68        0
    //  no simd      108      122        0
    fn mul(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       39        0
    //    simd3        0        3        0
    //    simd4       13       10        0
    // Totals...
    // yes simd       37       52        0
    //  no simd       76       88        0
    fn mul(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       34        0
    //    simd3        0        4        0
    //    simd4       15       11        0
    // Totals...
    // yes simd       30       49        0
    //  no simd       75       90        0
    fn mul(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOrthogonalOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       37       61        0
    //    simd3        0        7        0
    //    simd4       17       10        0
    // Totals...
    // yes simd       54       78        0
    //  no simd      105      122        0
    fn mul(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       15        0
    //    simd4        6        9        0
    // Totals...
    // yes simd       10       24        0
    //  no simd       28       51        0
    fn mul(self, other: DipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOrthogonalOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       55        0
    //    simd3        0        8        0
    //    simd4       13        5        0
    // Totals...
    // yes simd       45       68        0
    //  no simd       84       99        0
    fn mul(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        2        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2       12        0
    //  no simd        6       32        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       19        0
    fn mul(self, other: FlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       17        0
    //    simd3        3        5        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       13       25        0
    //  no simd       28       44        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPointAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       22        0
    //    simd3        0        1        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       12       25        0
    //  no simd       21       33        0
    fn mul(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       23        0
    //    simd3        0        3        0
    //    simd4       16       14        0
    // Totals...
    // yes simd       28       40        0
    //  no simd       76       88        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       17       29        0
    //  no simd       32       44        0
    fn mul(self, other: FlectorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       16        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       12       24        0
    //  no simd       36       48        0
    fn mul(self, other: FlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn mul(self, other: Horizon) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Infinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       19        0
    fn mul(self, other: Infinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       41        0
    //    simd3        0        4        0
    //    simd4        8        4        0
    // Totals...
    // yes simd       27       49        0
    //  no simd       51       69        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1       20        0
    //    simd4        5        5        0
    // Totals...
    // yes simd        6       25        0
    //  no simd       21       40        0
    fn mul(self, other: LineAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       27        0
    //    simd3        0        3        0
    //    simd4        3        0        0
    // Totals...
    // yes simd       12       30        0
    //  no simd       21       36        0
    fn mul(self, other: LineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       42        0
    //    simd3        0        6        0
    //    simd4       13        7        0
    // Totals...
    // yes simd       37       55        0
    //  no simd       76       88        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1       14        0
    //    simd4        8        9        0
    // Totals...
    // yes simd        9       23        0
    //  no simd       33       50        0
    fn mul(self, other: MotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       27        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       17       32        0
    //  no simd       32       47        0
    fn mul(self, other: MotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       74        0
    //    simd2        4        4        0
    //    simd3       40       53        0
    //    simd4       38       29        0
    // Totals...
    // yes simd      122      160        0
    //  no simd      320      357        0
    fn mul(self, other: MultiVector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircle> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       23        0
    //    simd3        1        3        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       18       29        0
    //  no simd       29       44        0
    fn mul(self, other: MysteryCircle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircleRotor> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       28        0
    //    simd3        0        5        0
    //    simd4        6        3        0
    // Totals...
    // yes simd       24       36        0
    //  no simd       42       55        0
    fn mul(self, other: MysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipole> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       23        0
    //    simd3        1        4        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       18       30        0
    //  no simd       29       47        0
    fn mul(self, other: MysteryDipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipoleInversion> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       34        0
    //    simd3        0        5        0
    //    simd4       12        8        0
    // Totals...
    // yes simd       28       47        0
    //  no simd       64       81        0
    fn mul(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorEven> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       34        0
    //    simd3        0        1        0
    //    simd4       14       13        0
    // Totals...
    // yes simd       34       48        0
    //  no simd       76       89        0
    fn mul(self, other: MysteryVersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorOdd> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       33        0
    //    simd3        0        1        0
    //    simd4       14       14        0
    // Totals...
    // yes simd       33       48        0
    //  no simd       75       92        0
    fn mul(self, other: MysteryVersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullCircleAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       20        0
    //    simd3        0        4        0
    //    simd4        4        1        0
    // Totals...
    // yes simd       11       25        0
    //  no simd       23       36        0
    fn mul(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1       18        0
    //    simd3        0        1        0
    //    simd4        5        4        0
    // Totals...
    // yes simd        6       23        0
    //  no simd       21       37        0
    fn mul(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleInversionAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        8        0
    //    simd4        8        9        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       33       44        0
    fn mul(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullSphereAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       18        0
    fn mul(self, other: NullSphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullVersorEvenAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       19        0
    //    simd3        0        4        0
    //    simd4        7        4        0
    // Totals...
    // yes simd       14       27        0
    //  no simd       35       47        0
    fn mul(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Origin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       22        0
    fn mul(self, other: Origin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       20        0
    //    simd3        2        4        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       15       27        0
    //  no simd       28       44        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<PlaneOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       21        0
    //    simd3        0        1        0
    //    simd4        4        3        0
    // Totals...
    // yes simd        9       25        0
    //  no simd       21       36        0
    fn mul(self, other: PlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       23        0
    //    simd3        0        4        0
    //    simd4        9        5        0
    // Totals...
    // yes simd       17       32        0
    //  no simd       44       55        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPointAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        9        0
    //    simd3        1        4        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        3       16        0
    //  no simd        8       33        0
    fn mul(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn mul(self, other: Scalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for AntiDipoleInversionOrthogonalOrigin {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       26        0
    //    simd3        0        2        0
    //    simd4        8        6        0
    // Totals...
    // yes simd       19       34        0
    //  no simd       43       56        0
    fn mul(self, other: Sphere) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        7        0
    //    simd3        1        3        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3       12        0
    //  no simd        8       24        0
    fn mul(self, other: SphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        0
    //    simd3        3        5        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       13       26        0
    //  no simd       28       45        0
    fn mul(self, other: SphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       64        0
    //    simd3        0       13        0
    //    simd4       32       19        0
    // Totals...
    // yes simd       64       96        0
    //  no simd      160      179        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAligningOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       63        0
    //    simd3        0        7        0
    //    simd4       19       12        0
    // Totals...
    // yes simd       59       82        0
    //  no simd      116      132        0
    fn mul(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       42        0
    //    simd3        0        9        0
    //    simd4       25       16        0
    // Totals...
    // yes simd       44       67        0
    //  no simd      119      133        0
    fn mul(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       41        0
    //    simd3        0        5        0
    //    simd4       13        8        0
    // Totals...
    // yes simd       37       54        0
    //  no simd       76       88        0
    fn mul(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       52        0
    //    simd3        0        4        0
    //    simd4       10        6        0
    // Totals...
    // yes simd       42       62        0
    //  no simd       72       88        0
    fn mul(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOrthogonalOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       42        0
    //    simd3        0       11        0
    //    simd4       26       15        0
    // Totals...
    // yes simd       42       68        0
    //  no simd      120      135        0
    fn mul(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       57        0
    //    simd3        0        8        0
    //    simd4       32       24        0
    // Totals...
    // yes simd       64       89        0
    //  no simd      160      177        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       46        0
    //    simd3        0        5        0
    //    simd4       23       18        0
    // Totals...
    // yes simd       47       69        0
    //  no simd      116      133        0
    fn mul(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddOrthogonalOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       62        0
    //    simd3        0        5        0
    //    simd4       19       14        0
    // Totals...
    // yes simd       59       81        0
    //  no simd      116      133        0
    fn mul(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Neg for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn neg(self) -> Self::Output {
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Not for AntiDipoleInversionOrthogonalOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiCircleOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<AntiCircleRotor> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(other[e45]) * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<AntiCircleRotorAligningOrigin> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<AntiCircleRotorAligningOriginAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<AntiCircleRotorAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<AntiCircleRotorOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<AntiDipoleInversion> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        4        2        0
    //  no simd       11        8        0
    fn sub(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz() - other.group0(),
            // e415, e425, e435, e321
            (self.group1() - other.group1().xyz()).with_w(other[e321]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e4
            self.group2() - other.group2(),
            // e1, e2, e3, e5
            other.group3().xyz().with_w(self[e5] - other[e5]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        1        0        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        8       11        0
    fn sub(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            (self.group1() - other.group0().xyz()).with_w(other[e321]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e4
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
            // e1, e2, e3, e5
            other.group2().xyz().with_w(self[e5] - other[e5]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        1        0        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        7        6        0
    fn sub(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz() - other.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(other[e321] * -1.0),
            // e235, e315, e125, e4
            self.group2() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
            // e1, e2, e3, e5
            other.group1().yzw().with_w(self[e5]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOrthogonalOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       11        0        0
    fn sub(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0() - other.group0(),
            // e415, e425, e435
            self.group1() - other.group1(),
            // e235, e315, e125, e4
            self.group2() - other.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiDipoleInversionOrthogonalOrigin> for AntiDipoleInversionOrthogonalOrigin {
    fn sub_assign(&mut self, other: AntiDipoleInversionOrthogonalOrigin) {
        *self = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0() - other.group0(),
            // e415, e425, e435
            self.group1() - other.group1(),
            // e235, e315, e125, e4
            self.group2() - other.group2(),
        );
    }
}
impl std::ops::Sub<AntiDipoleOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        3        1        0
    fn sub(self, other: AntiDipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz() - other.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(other[e321] * -1.0),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(self[e5]),
        );
    }
}
impl std::ops::Sub<AntiDualNum> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<AntiFlatOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(other[e321] * -1.0),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(self[e5]),
        );
    }
}
impl std::ops::Sub<AntiFlatPoint> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        4        4        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(other[e321] * -1.0),
            // e235, e315, e125, e4
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(self[e5]),
        );
    }
}
impl std::ops::Sub<AntiFlector> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        4        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        5        8        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(other[e321] * -1.0),
            // e235, e315, e125, e4
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
            // e1, e2, e3, e5
            other.group1().xyz().with_w(self[e5] - other[e5]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<AntiFlectorOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(other[e321] * -1.0),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            other.group0().yzw().with_w(self[e5]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<AntiLine> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<AntiLineOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<AntiMotor> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<AntiMotorOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<AntiMysteryCircleRotor> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<AntiMysteryDipoleInversion> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        3        8        0
    fn sub(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            (self.group1() - other.group0().xyz()).with_w(other[e321]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            other.group1().with_w(self[e5]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<AntiPlane> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        1        4        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            other.group0().xyz().with_w(self[e5] - other[e5]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<AntiPlaneOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiPlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            other.group0().with_w(self[e5]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<AntiScalar> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e4
            self.group1().with_w(self[e4]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl std::ops::Sub<AntiSphereOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        4        5        0
    fn sub(self, other: AntiSphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
            // e235, e315, e125, e4
            self.group2() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
            // e1, e2, e3, e5
            other.group0().xyz().with_w(self[e5]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<AntiVersorEvenOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<Circle> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        2        0        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        3        4        0
    //  no simd       10        7        0
    fn sub(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz() - other.group0(),
            // e415, e425, e435, e321
            (self.group1() - other.group1().xyz()).with_w(other[e321]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e4
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(self[e5]),
        );
    }
}
impl std::ops::Sub<CircleAligningOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        6        0
    //  no simd       11        6        0
    fn sub(self, other: CircleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435
            self.group1() - other.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::SubAssign<CircleAligningOrigin> for AntiDipoleInversionOrthogonalOrigin {
    fn sub_assign(&mut self, other: CircleAligningOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435
            self.group1() - other.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<CircleAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        1        0        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        7        7        0
    fn sub(self, other: CircleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            (self.group1() - other.group0().xyz()).with_w(other[e321]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e4
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(self[e5]),
        );
    }
}
impl std::ops::Sub<CircleAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        8        6        0
    fn sub(self, other: CircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::SubAssign<CircleAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    fn sub_assign(&mut self, other: CircleAtOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<CircleOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        7        3        0
    fn sub(self, other: CircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435
            self.group1() - other.group1(),
            // e235, e315, e125, e4
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<CircleOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    fn sub_assign(&mut self, other: CircleOnOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435
            self.group1() - other.group1(),
            // e235, e315, e125, e4
            self.group2(),
        );
    }
}
impl std::ops::Sub<CircleOrthogonalOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        7        4        0
    fn sub(self, other: CircleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz() - other.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(other[e321] * -1.0),
            // e235, e315, e125, e4
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(self[e5]),
        );
    }
}
impl std::ops::Sub<CircleRotor> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        2        0        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3        5        0
    //  no simd       10       11        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0().xyz() - other.group0()).with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            (self.group1() - other.group1().xyz()).with_w(other[e321]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2().xyz().with_w(self[e5]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        1        0        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        3        7        0
    //  no simd       11       10        0
    fn sub(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            (self.group0().xyz() - other.group0()).with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e4
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group1().with_w(self[e4]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2().xyz().with_w(self[e5]),
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOriginAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        8       10        0
    fn sub(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e4
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group1().with_w(self[e4]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2().xyz().with_w(self[e5]),
        );
    }
}
impl std::ops::Sub<CircleRotorAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        1        0        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        7       11        0
    fn sub(self, other: CircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            (self.group1() - other.group0().xyz()).with_w(other[e321]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2().xyz().with_w(self[e5]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
        );
    }
}
impl std::ops::Sub<CircleRotorOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        1        0        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        7        7        0
    fn sub(self, other: CircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            (self.group0().xyz() - other.group0().xyz()).with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e4
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group1().with_w(self[e4]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl std::ops::Sub<Dipole> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(other[e45]) * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group2() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<DipoleAligningOrigin> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<DipoleAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<DipoleAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<DipoleInversion> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(other[e45]) * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<DipoleInversionAligningOrigin> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<DipoleInversionAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<DipoleInversionAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<DipoleInversionOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<DipoleInversionOrthogonalOrigin> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<DipoleOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<DipoleOrthogonalOrigin> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            other.group2() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<DualNum> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        1        4        0
    fn sub(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e4
            self.group1().with_w(self[e4] - other[e4]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl std::ops::Sub<FlatOrigin> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<FlatPoint> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<FlatPointAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<Flector> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<FlectorAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<FlectorOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<Horizon> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<Infinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0() + Simd32x3::from(0.0).with_w(other[e5] * -1.0),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<Infinity> for AntiDipoleInversionOrthogonalOrigin {
    fn sub_assign(&mut self, other: Infinity) {
        use crate::elements::*;
        *self = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0() + Simd32x3::from(0.0).with_w(other[e5] * -1.0),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            self.group2(),
        );
    }
}
impl std::ops::Sub<Line> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        7        3        0
    fn sub(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0(),
            // e415, e425, e435
            self.group1() - other.group0(),
            // e235, e315, e125, e4
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::SubAssign<Line> for AntiDipoleInversionOrthogonalOrigin {
    fn sub_assign(&mut self, other: Line) {
        use crate::elements::*;
        *self = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0(),
            // e415, e425, e435
            self.group1() - other.group0(),
            // e235, e315, e125, e4
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<LineAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        4        3        0
    fn sub(self, other: LineAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0(),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::SubAssign<LineAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    fn sub_assign(&mut self, other: LineAtInfinity) {
        use crate::elements::*;
        *self = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0(),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<LineOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn sub(self, other: LineOnOrigin) -> Self::Output {
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0(),
            // e415, e425, e435
            self.group1() - other.group0(),
            // e235, e315, e125, e4
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<LineOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    fn sub_assign(&mut self, other: LineOnOrigin) {
        *self = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0(),
            // e415, e425, e435
            self.group1() - other.group0(),
            // e235, e315, e125, e4
            self.group2(),
        );
    }
}
impl std::ops::Sub<Motor> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        7        0
    fn sub(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e4
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group1().with_w(self[e4]),
            // e235, e315, e125, e5
            self.group2().xyz().with_w(self[e5]) - other.group1(),
        );
    }
}
impl std::ops::Sub<MotorAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        4        0
    fn sub(self, other: MotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0() + Simd32x3::from(0.0).with_w(other[e5] * -1.0),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::SubAssign<MotorAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    fn sub_assign(&mut self, other: MotorAtInfinity) {
        use crate::elements::*;
        *self = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0() + Simd32x3::from(0.0).with_w(other[e5] * -1.0),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<MotorOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        4        7        0
    fn sub(self, other: MotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e4
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group1().with_w(self[e4]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl std::ops::Sub<MultiVector> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        1        0
    //    simd2        0        1        0
    //    simd3        3        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        5        8        0
    //  no simd       11       25        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            other.group0() * Simd32x2::from(-1.0),
            // e1, e2, e3, e4
            other.group1().xyz().with_w(self[e4] - other[e4]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            other.group3() * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group4() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group5() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            (self.group1() - other.group6().xyz()).with_w(other[e321]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
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
impl std::ops::Sub<MysteryCircle> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        3        4        0
    fn sub(self, other: MysteryCircle) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            (self.group1() - other.group0().xyz()).with_w(other[e321]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(self[e5]),
        );
    }
}
impl std::ops::Sub<MysteryCircleRotor> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        3        8        0
    fn sub(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            (self.group1() - other.group0().xyz()).with_w(other[e321]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
        );
    }
}
impl std::ops::Sub<MysteryDipole> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<MysteryDipoleInversion> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<MysteryVersorEven> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        3       12        0
    fn sub(self, other: MysteryVersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            (self.group1() - other.group1().xyz()).with_w(other[e321]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
            // e1, e2, e3, e4
            other.group0().yzw().with_w(self[e4]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<MysteryVersorOdd> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<NullCircleAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        4        3        0
    fn sub(self, other: NullCircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<NullCircleAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    fn sub_assign(&mut self, other: NullCircleAtOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            self.group2(),
        );
    }
}
impl std::ops::Sub<NullDipoleAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<NullDipoleInversionAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<NullSphereAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<NullVersorEvenAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        4        0
    fn sub(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            self.group2() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::SubAssign<NullVersorEvenAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    fn sub_assign(&mut self, other: NullVersorEvenAtOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, 0.0]) + self.group0(),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            self.group2() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::Sub<Origin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0(),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            self.group2() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::SubAssign<Origin> for AntiDipoleInversionOrthogonalOrigin {
    fn sub_assign(&mut self, other: Origin) {
        use crate::elements::*;
        *self = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0(),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            self.group2() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::Sub<Plane> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<PlaneOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<RoundPoint> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        5        5        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
            // e235, e315, e125, e4
            self.group2() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
            // e1, e2, e3, e5
            other.group0().xyz().with_w(self[e5] - other[e5]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<RoundPointAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        8        2        0
    fn sub(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0() + Simd32x3::from(0.0).with_w(other[e5] * -1.0),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            self.group2() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::SubAssign<RoundPointAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    fn sub_assign(&mut self, other: RoundPointAtOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0() + Simd32x3::from(0.0).with_w(other[e5] * -1.0),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            self.group2() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
        );
    }
}
impl std::ops::Sub<Scalar> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<Sphere> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<SphereAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<SphereOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<VersorEven> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        4        3        0
    //  no simd       11       12        0
    fn sub(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0().xyz() - other.group0().xyz()).with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            (self.group1() - other.group1().xyz()).with_w(other[e321]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e5
            self.group2().xyz().with_w(self[e5]) - other.group2(),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(self[e4] - other[e4]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<VersorEvenAligningOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd       11        4        0
    fn sub(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            (self.group0().xyz() - other.group0().xyz()).with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e4
            self.group1().with_w(self[e4]) - other.group1(),
            // e235, e315, e125, e5
            self.group2().xyz().with_w(self[e5]) - other.group2(),
        );
    }
}
impl std::ops::Sub<VersorEvenAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        7       12        0
    fn sub(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            (self.group1() - other.group1().xyz()).with_w(other[e321]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e5
            self.group2().xyz().with_w(self[e5]) - other.group2(),
            // e1, e2, e3, e4
            other.group0().yzw().with_w(self[e4]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<VersorEvenAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn sub(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0() - other.group0().xyz().with_w(other[e5]),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            self.group2() - other.group1().xyz().with_w(other[e4]),
        );
    }
}
impl std::ops::SubAssign<VersorEvenAtOrigin> for AntiDipoleInversionOrthogonalOrigin {
    fn sub_assign(&mut self, other: VersorEvenAtOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0() - other.group0().xyz().with_w(other[e5]),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            self.group2() - other.group1().xyz().with_w(other[e4]),
        );
    }
}
impl std::ops::Sub<VersorEvenOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        7        4        0
    fn sub(self, other: VersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            (self.group0().xyz() - other.group0().xyz()).with_w(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e4
            self.group1().with_w(self[e4]) - other.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]),
        );
    }
}
impl std::ops::Sub<VersorEvenOrthogonalOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        1        0
    //    simd3        1        0        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        8        5        0
    fn sub(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz() - other.group0().xyz(),
            // e415, e425, e435, e321
            self.group1().with_w(other[e321] * -1.0),
            // e235, e315, e125, e4
            self.group2() - other.group1().xyz().with_w(other[e4]),
            // e1, e2, e3, e5
            other.group2().xyz().with_w(self[e5] - other[e5]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<VersorOdd> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().xyz().with_w(other[e45]) * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<VersorOddAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            other.group0().yzw() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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
impl std::ops::Sub<VersorOddOrthogonalOrigin> for AntiDipoleInversionOrthogonalOrigin {
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
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
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

impl TryFrom<AntiDipoleInversion> for AntiDipoleInversionOrthogonalOrigin {
    type Error = String;
    fn try_from(anti_dipole_inversion: AntiDipoleInversion) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
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
            let mut error = "Elements from AntiDipoleInversion do not fit into AntiDipoleInversionOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            anti_dipole_inversion.group0().with_w(anti_dipole_inversion[e5]),
            // e415, e425, e435
            anti_dipole_inversion.group1().xyz(),
            // e235, e315, e125, e4
            anti_dipole_inversion.group2(),
        ));
    }
}

impl TryFrom<AntiDipoleInversionAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Error = String;
    fn try_from(anti_dipole_inversion_at_infinity: AntiDipoleInversionAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversionAtInfinity do not fit into AntiDipoleInversionOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x3::from(0.0).with_w(anti_dipole_inversion_at_infinity[e5]),
            // e415, e425, e435
            anti_dipole_inversion_at_infinity.group0().xyz(),
            // e235, e315, e125, e4
            anti_dipole_inversion_at_infinity.group1().with_w(0.0),
        ));
    }
}

impl TryFrom<AntiDipoleInversionOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Error = String;
    fn try_from(anti_dipole_inversion_on_origin: AntiDipoleInversionOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_on_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_on_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_on_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversionOnOrigin do not fit into AntiDipoleInversionOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            anti_dipole_inversion_on_origin.group0().xyz().with_w(0.0),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(anti_dipole_inversion_on_origin[e4]),
        ));
    }
}

impl TryFrom<AntiDipoleOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Error = String;
    fn try_from(anti_dipole_on_origin: AntiDipoleOnOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleOnOrigin do not fit into AntiDipoleInversionOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            anti_dipole_on_origin.group0().xyz().with_w(0.0),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiFlatPoint> for AntiDipoleInversionOrthogonalOrigin {
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
            let mut error = "Elements from AntiFlatPoint do not fit into AntiDipoleInversionOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from(0.0),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e4
            anti_flat_point.group0().xyz().with_w(0.0),
        ));
    }
}

impl TryFrom<AntiFlector> for AntiDipoleInversionOrthogonalOrigin {
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
            let mut error = "Elements from AntiFlector do not fit into AntiDipoleInversionOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x3::from(0.0).with_w(anti_flector[e5]),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e4
            anti_flector.group0().xyz().with_w(0.0),
        ));
    }
}

impl TryFrom<AntiMysteryDipoleInversion> for AntiDipoleInversionOrthogonalOrigin {
    type Error = String;
    fn try_from(anti_mystery_dipole_inversion: AntiMysteryDipoleInversion) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_mystery_dipole_inversion[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_mystery_dipole_inversion[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_mystery_dipole_inversion[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_mystery_dipole_inversion[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMysteryDipoleInversion do not fit into AntiDipoleInversionOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from(0.0),
            // e415, e425, e435
            anti_mystery_dipole_inversion.group0().xyz(),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiPlane> for AntiDipoleInversionOrthogonalOrigin {
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
            let mut error = "Elements from AntiPlane do not fit into AntiDipoleInversionOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x3::from(0.0).with_w(anti_plane[e5]),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiSphereOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Error = String;
    fn try_from(anti_sphere_on_origin: AntiSphereOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_sphere_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_sphere_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_sphere_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiSphereOnOrigin do not fit into AntiDipoleInversionOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from(0.0),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(anti_sphere_on_origin[e4]),
        ));
    }
}

impl TryFrom<Circle> for AntiDipoleInversionOrthogonalOrigin {
    type Error = String;
    fn try_from(circle: Circle) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Circle do not fit into AntiDipoleInversionOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            circle.group0().with_w(0.0),
            // e415, e425, e435
            circle.group1().xyz(),
            // e235, e315, e125, e4
            circle.group2().with_w(0.0),
        ));
    }
}

impl TryFrom<CircleAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Error = String;
    fn try_from(circle_at_infinity: CircleAtInfinity) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleAtInfinity do not fit into AntiDipoleInversionOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from(0.0),
            // e415, e425, e435
            circle_at_infinity.group0().xyz(),
            // e235, e315, e125, e4
            circle_at_infinity.group1().with_w(0.0),
        ));
    }
}

impl TryFrom<CircleOrthogonalOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Error = String;
    fn try_from(circle_orthogonal_origin: CircleOrthogonalOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_orthogonal_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleOrthogonalOrigin do not fit into AntiDipoleInversionOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            circle_orthogonal_origin.group0().xyz().with_w(0.0),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e4
            circle_orthogonal_origin.group1().with_w(0.0),
        ));
    }
}

impl TryFrom<CircleRotor> for AntiDipoleInversionOrthogonalOrigin {
    type Error = String;
    fn try_from(circle_rotor: CircleRotor) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
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
            let mut error = "Elements from CircleRotor do not fit into AntiDipoleInversionOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            circle_rotor.group0().with_w(0.0),
            // e415, e425, e435
            circle_rotor.group1().xyz(),
            // e235, e315, e125, e4
            circle_rotor.group2().xyz().with_w(0.0),
        ));
    }
}

impl TryFrom<CircleRotorAligningOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Error = String;
    fn try_from(circle_rotor_aligning_origin: CircleRotorAligningOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor_aligning_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotorAligningOrigin do not fit into AntiDipoleInversionOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            circle_rotor_aligning_origin.group0().with_w(0.0),
            // e415, e425, e435
            circle_rotor_aligning_origin.group1(),
            // e235, e315, e125, e4
            circle_rotor_aligning_origin.group2().xyz().with_w(0.0),
        ));
    }
}

impl TryFrom<CircleRotorAligningOriginAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Error = String;
    fn try_from(circle_rotor_aligning_origin_at_infinity: CircleRotorAligningOriginAtInfinity) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor_aligning_origin_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotorAligningOriginAtInfinity do not fit into AntiDipoleInversionOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from(0.0),
            // e415, e425, e435
            circle_rotor_aligning_origin_at_infinity.group0(),
            // e235, e315, e125, e4
            circle_rotor_aligning_origin_at_infinity.group1().xyz().with_w(0.0),
        ));
    }
}

impl TryFrom<CircleRotorAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
    type Error = String;
    fn try_from(circle_rotor_at_infinity: CircleRotorAtInfinity) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
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
            let mut error = "Elements from CircleRotorAtInfinity do not fit into AntiDipoleInversionOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from(0.0),
            // e415, e425, e435
            circle_rotor_at_infinity.group0().xyz(),
            // e235, e315, e125, e4
            circle_rotor_at_infinity.group1().xyz().with_w(0.0),
        ));
    }
}

impl TryFrom<CircleRotorOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
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
        if fail {
            let mut error = "Elements from CircleRotorOnOrigin do not fit into AntiDipoleInversionOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            circle_rotor_on_origin.group0().xyz().with_w(0.0),
            // e415, e425, e435
            circle_rotor_on_origin.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<DualNum> for AntiDipoleInversionOrthogonalOrigin {
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
            let mut error = "Elements from DualNum do not fit into AntiDipoleInversionOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from(0.0),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(dual_num[e4]),
        ));
    }
}

impl TryFrom<Motor> for AntiDipoleInversionOrthogonalOrigin {
    type Error = String;
    fn try_from(motor: Motor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = motor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Motor do not fit into AntiDipoleInversionOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x3::from(0.0).with_w(motor[e5]),
            // e415, e425, e435
            motor.group0().xyz(),
            // e235, e315, e125, e4
            motor.group1().xyz().with_w(0.0),
        ));
    }
}

impl TryFrom<MotorOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Error = String;
    fn try_from(motor_on_origin: MotorOnOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = motor_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MotorOnOrigin do not fit into AntiDipoleInversionOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from(0.0),
            // e415, e425, e435
            motor_on_origin.group0().xyz(),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<MultiVector> for AntiDipoleInversionOrthogonalOrigin {
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
        let el = multi_vector[20];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
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
            let mut error = "Elements from MultiVector do not fit into AntiDipoleInversionOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            multi_vector.group7().with_w(multi_vector[e5]),
            // e415, e425, e435
            multi_vector.group6().xyz(),
            // e235, e315, e125, e4
            multi_vector.group8().with_w(multi_vector[e4]),
        ));
    }
}

impl TryFrom<MysteryCircle> for AntiDipoleInversionOrthogonalOrigin {
    type Error = String;
    fn try_from(mystery_circle: MysteryCircle) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_circle[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryCircle do not fit into AntiDipoleInversionOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from(0.0),
            // e415, e425, e435
            mystery_circle.group0().xyz(),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<MysteryCircleRotor> for AntiDipoleInversionOrthogonalOrigin {
    type Error = String;
    fn try_from(mystery_circle_rotor: MysteryCircleRotor) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_circle_rotor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
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
            let mut error = "Elements from MysteryCircleRotor do not fit into AntiDipoleInversionOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from(0.0),
            // e415, e425, e435
            mystery_circle_rotor.group0().xyz(),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<MysteryVersorEven> for AntiDipoleInversionOrthogonalOrigin {
    type Error = String;
    fn try_from(mystery_versor_even: MysteryVersorEven) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_versor_even[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_even[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_even[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_even[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_even[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryVersorEven do not fit into AntiDipoleInversionOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from(0.0),
            // e415, e425, e435
            mystery_versor_even.group1().xyz(),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<RoundPoint> for AntiDipoleInversionOrthogonalOrigin {
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
        if fail {
            let mut error = "Elements from RoundPoint do not fit into AntiDipoleInversionOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x3::from(0.0).with_w(round_point[e5]),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(round_point[e4]),
        ));
    }
}

impl TryFrom<VersorEven> for AntiDipoleInversionOrthogonalOrigin {
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
        if fail {
            let mut error = "Elements from VersorEven do not fit into AntiDipoleInversionOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from([versor_even[e423], versor_even[e431], versor_even[e412], versor_even[e5]]),
            // e415, e425, e435
            versor_even.group1().xyz(),
            // e235, e315, e125, e4
            Simd32x4::from([versor_even[e235], versor_even[e315], versor_even[e125], versor_even[e4]]),
        ));
    }
}

impl TryFrom<VersorEvenAligningOrigin> for AntiDipoleInversionOrthogonalOrigin {
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
        if fail {
            let mut error = "Elements from VersorEvenAligningOrigin do not fit into AntiDipoleInversionOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from([
                versor_even_aligning_origin[e423],
                versor_even_aligning_origin[e431],
                versor_even_aligning_origin[e412],
                versor_even_aligning_origin[e5],
            ]),
            // e415, e425, e435
            versor_even_aligning_origin.group1().xyz(),
            // e235, e315, e125, e4
            Simd32x4::from([
                versor_even_aligning_origin[e235],
                versor_even_aligning_origin[e315],
                versor_even_aligning_origin[e125],
                versor_even_aligning_origin[e4],
            ]),
        ));
    }
}

impl TryFrom<VersorEvenAtInfinity> for AntiDipoleInversionOrthogonalOrigin {
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
        let el = versor_even_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAtInfinity do not fit into AntiDipoleInversionOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x3::from(0.0).with_w(versor_even_at_infinity[e5]),
            // e415, e425, e435
            versor_even_at_infinity.group1().xyz(),
            // e235, e315, e125, e4
            versor_even_at_infinity.group2().xyz().with_w(0.0),
        ));
    }
}

impl TryFrom<VersorEvenOnOrigin> for AntiDipoleInversionOrthogonalOrigin {
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
        if fail {
            let mut error = "Elements from VersorEvenOnOrigin do not fit into AntiDipoleInversionOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            versor_even_on_origin.group0().xyz().with_w(0.0),
            // e415, e425, e435
            versor_even_on_origin.group1().xyz(),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(versor_even_on_origin[e4]),
        ));
    }
}

impl TryFrom<VersorEvenOrthogonalOrigin> for AntiDipoleInversionOrthogonalOrigin {
    type Error = String;
    fn try_from(versor_even_orthogonal_origin: VersorEvenOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_orthogonal_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenOrthogonalOrigin do not fit into AntiDipoleInversionOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from([
                versor_even_orthogonal_origin[e423],
                versor_even_orthogonal_origin[e431],
                versor_even_orthogonal_origin[e412],
                versor_even_orthogonal_origin[e5],
            ]),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e4
            Simd32x4::from([
                versor_even_orthogonal_origin[e235],
                versor_even_orthogonal_origin[e315],
                versor_even_orthogonal_origin[e125],
                versor_even_orthogonal_origin[e4],
            ]),
        ));
    }
}
