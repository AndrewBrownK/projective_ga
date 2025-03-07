use crate::traits::GeometricProduct;
use crate::traits::RightDual;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 329
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         1       1       0
//  Average:         7      12       0
//  Maximum:       142     174       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       4       0
//  Average:        19      23       0
//  Maximum:       324     355       0
impl std::ops::Add<AntiCircleOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiCircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotor> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(other[e45]),
            // e15, e25, e35
            other.group2().xyz(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(0.0),
            // e15, e25, e35
            other.group2().xyz(),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOriginAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1().xyz(),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            other.group1().xyz(),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().xyz().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiDipoleInversion> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       11        0        0
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0(),
            // e415, e425, e435, e321
            other.group1() + self.group0(),
            // e235, e315, e125, e4
            (self.group1() + other.group2().xyz()).with_w(other[e4]),
            // e1, e2, e3, e5
            other.group3() + self.group2(),
        );
    }
}
impl std::ops::Add<AntiDipoleInversionAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       11        0        0
    fn add(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            other.group0() + self.group0(),
            // e235, e315, e125
            other.group1() + self.group1(),
            // e1, e2, e3, e5
            other.group2() + self.group2(),
        );
    }
}
impl std::ops::AddAssign<AntiDipoleInversionAtInfinity> for AntiDipoleInversionAtInfinity {
    fn add_assign(&mut self, other: AntiDipoleInversionAtInfinity) {
        *self = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            other.group0() + self.group0(),
            // e235, e315, e125
            other.group1() + self.group1(),
            // e1, e2, e3, e5
            other.group2() + self.group2(),
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321]),
            // e235, e315, e125, e4
            self.group1().with_w(other[e4]),
            // e1, e2, e3, e5
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn add(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0().xyz(),
            // e415, e425, e435, e321
            (other.group1() + self.group0().xyz()).with_w(self[e321]),
            // e235, e315, e125, e4
            (self.group1() + other.group2().xyz()).with_w(other[e4]),
            // e1, e2, e3, e5
            self.group2() + Simd32x3::from(0.0).with_w(other[e5]),
        );
    }
}
impl std::ops::Add<AntiDipoleOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: AntiDipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321]),
            // e235, e315, e125, e4
            self.group1().with_w(0.0),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiDualNum> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiFlatOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321]),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<AntiFlatOrigin> for AntiDipoleInversionAtInfinity {
    fn add_assign(&mut self, other: AntiFlatOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321]),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiFlatPoint> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321]),
            // e235, e315, e125
            self.group1() + other.group0().xyz(),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<AntiFlatPoint> for AntiDipoleInversionAtInfinity {
    fn add_assign(&mut self, other: AntiFlatPoint) {
        use crate::elements::*;
        *self = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321]),
            // e235, e315, e125
            self.group1() + other.group0().xyz(),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiFlector> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       11        0        0
    fn add(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321]),
            // e235, e315, e125
            self.group1() + other.group0().xyz(),
            // e1, e2, e3, e5
            self.group2() + other.group1(),
        );
    }
}
impl std::ops::AddAssign<AntiFlector> for AntiDipoleInversionAtInfinity {
    fn add_assign(&mut self, other: AntiFlector) {
        use crate::elements::*;
        *self = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321]),
            // e235, e315, e125
            self.group1() + other.group0().xyz(),
            // e1, e2, e3, e5
            self.group2() + other.group1(),
        );
    }
}
impl std::ops::Add<AntiFlectorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321]),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]) + self.group2(),
        );
    }
}
impl std::ops::AddAssign<AntiFlectorOnOrigin> for AntiDipoleInversionAtInfinity {
    fn add_assign(&mut self, other: AntiFlectorOnOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321]),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Add<AntiLine> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiLineOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiLineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiMotor> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1().xyz(),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<AntiMotorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiMysteryCircleRotor> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiMysteryDipoleInversion> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() + other.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            (other.group1() + self.group2().xyz()).with_w(self[e5]),
        );
    }
}
impl std::ops::AddAssign<AntiMysteryDipoleInversion> for AntiDipoleInversionAtInfinity {
    fn add_assign(&mut self, other: AntiMysteryDipoleInversion) {
        use crate::elements::*;
        *self = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() + other.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            (other.group1() + self.group2().xyz()).with_w(self[e5]),
        );
    }
}
impl std::ops::Add<AntiPlane> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: AntiPlane) -> Self::Output {
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            self.group2() + other.group0(),
        );
    }
}
impl std::ops::AddAssign<AntiPlane> for AntiDipoleInversionAtInfinity {
    fn add_assign(&mut self, other: AntiPlane) {
        *self = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            self.group2() + other.group0(),
        );
    }
}
impl std::ops::Add<AntiPlaneOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: AntiPlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            (other.group0() + self.group2().xyz()).with_w(self[e5]),
        );
    }
}
impl std::ops::AddAssign<AntiPlaneOnOrigin> for AntiDipoleInversionAtInfinity {
    fn add_assign(&mut self, other: AntiPlaneOnOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            (other.group0() + self.group2().xyz()).with_w(self[e5]),
        );
    }
}
impl std::ops::Add<AntiScalar> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([other[e12345], self[e1], self[e2], self[e3]]),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1().with_w(self[e5]),
        );
    }
}
impl std::ops::Add<AntiSphereOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: AntiSphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            self.group1().with_w(other[e4]),
            // e1, e2, e3, e5
            self.group2() + other.group0().xyz().with_w(0.0),
        );
    }
}
impl std::ops::Add<AntiVersorEvenOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().xyz().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Circle> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: Circle) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0(),
            // e415, e425, e435, e321
            self.group0() + other.group1(),
            // e235, e315, e125, e4
            (self.group1() + other.group2()).with_w(0.0),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Add<CircleAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: CircleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0(),
            // e415, e425, e435, e321
            (other.group1() + self.group0().xyz()).with_w(self[e321]),
            // e235, e315, e125, e4
            (self.group1() + other.group2()).with_w(0.0),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Add<CircleAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: CircleAtInfinity) -> Self::Output {
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() + other.group0(),
            // e235, e315, e125
            self.group1() + other.group1(),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<CircleAtInfinity> for AntiDipoleInversionAtInfinity {
    fn add_assign(&mut self, other: CircleAtInfinity) {
        *self = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() + other.group0(),
            // e235, e315, e125
            self.group1() + other.group1(),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Add<CircleAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: CircleAtOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0(),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            (self.group1() + other.group1()).with_w(0.0),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Add<CircleOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: CircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0(),
            // e415, e425, e435, e321
            (other.group1() + self.group0().xyz()).with_w(self[e321]),
            // e235, e315, e125, e4
            self.group1().with_w(0.0),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Add<CircleOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: CircleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321]),
            // e235, e315, e125, e4
            (self.group1() + other.group1()).with_w(0.0),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Add<CircleRotor> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0().with_w(other[e12345]),
            // e415, e425, e435, e321
            self.group0() + other.group1(),
            // e235, e315, e125, e5
            (self.group1() + other.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
        );
    }
}
impl std::ops::Add<CircleRotorAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0().with_w(other[e12345]),
            // e415, e425, e435, e321
            (other.group1() + self.group0().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            (self.group1() + other.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
        );
    }
}
impl std::ops::Add<CircleRotorAligningOriginAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([other[e12345], self[e1], self[e2], self[e3]]),
            // e415, e425, e435, e321
            (other.group0() + self.group0().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            (self.group1() + other.group1().xyz()).with_w(self[e5]),
        );
    }
}
impl std::ops::Add<CircleRotorAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: CircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([other[e12345], self[e1], self[e2], self[e3]]),
            // e415, e425, e435, e321
            self.group0() + other.group0(),
            // e235, e315, e125, e5
            (self.group1() + other.group1().xyz()).with_w(self[e5]),
        );
    }
}
impl std::ops::Add<CircleRotorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: CircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0(),
            // e415, e425, e435, e321
            (other.group1() + self.group0().xyz()).with_w(self[e321]),
            // e235, e315, e125, e5
            self.group1().with_w(self[e5]),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
        );
    }
}
impl std::ops::Add<Dipole> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(other[e45]),
            // e15, e25, e35
            other.group2(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(0.0),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleInversion> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(other[e45]),
            // e15, e25, e35
            other.group2().xyz(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            other.group1().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().xyz().with_w(0.0),
            // e15, e25, e35
            other.group1().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            other.group1(),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleInversionOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().xyz().with_w(0.0),
            // e15, e25, e35
            other.group2().xyz(),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(0.0),
            // e15, e25, e35
            other.group2(),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DualNum> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    fn add(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(other[e12345]),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1().with_w(self[e5]),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(other[e4]),
        );
    }
}
impl std::ops::Add<FlatOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<FlatPoint> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            other.group0().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<FlatPointAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: FlatPointAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group0(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Flector> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            other.group0().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<FlectorAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group0().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<FlectorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Horizon> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<Infinity> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            self.group2() + Simd32x3::from(0.0).with_w(other[e5]),
        );
    }
}
impl std::ops::AddAssign<Infinity> for AntiDipoleInversionAtInfinity {
    fn add_assign(&mut self, other: Infinity) {
        use crate::elements::*;
        *self = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            self.group2() + Simd32x3::from(0.0).with_w(other[e5]),
        );
    }
}
impl std::ops::Add<Line> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            (other.group0() + self.group0().xyz()).with_w(self[e321]),
            // e235, e315, e125
            self.group1() + other.group1(),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<Line> for AntiDipoleInversionAtInfinity {
    fn add_assign(&mut self, other: Line) {
        use crate::elements::*;
        *self = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            (other.group0() + self.group0().xyz()).with_w(self[e321]),
            // e235, e315, e125
            self.group1() + other.group1(),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Add<LineAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: LineAtInfinity) -> Self::Output {
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1() + other.group0(),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<LineAtInfinity> for AntiDipoleInversionAtInfinity {
    fn add_assign(&mut self, other: LineAtInfinity) {
        *self = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1() + other.group0(),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Add<LineOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: LineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            (other.group0() + self.group0().xyz()).with_w(self[e321]),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<LineOnOrigin> for AntiDipoleInversionAtInfinity {
    fn add_assign(&mut self, other: LineOnOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            (other.group0() + self.group0().xyz()).with_w(self[e321]),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Add<Motor> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([other[e12345], self[e1], self[e2], self[e3]]),
            // e415, e425, e435, e321
            self.group0() + other.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            other.group1() + self.group1().with_w(self[e5]),
        );
    }
}
impl std::ops::Add<MotorAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: MotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1() + other.group0().xyz(),
            // e1, e2, e3, e5
            self.group2() + Simd32x3::from(0.0).with_w(other[e5]),
        );
    }
}
impl std::ops::AddAssign<MotorAtInfinity> for AntiDipoleInversionAtInfinity {
    fn add_assign(&mut self, other: MotorAtInfinity) {
        use crate::elements::*;
        *self = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1() + other.group0().xyz(),
            // e1, e2, e3, e5
            self.group2() + Simd32x3::from(0.0).with_w(other[e5]),
        );
    }
}
impl std::ops::Add<MotorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: MotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([other[e12345], self[e1], self[e2], self[e3]]),
            // e415, e425, e435, e321
            self.group0() + other.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            self.group1().with_w(self[e5]),
        );
    }
}
impl std::ops::Add<MultiVector> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       11        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            other.group0(),
            // e1, e2, e3, e4
            (self.group2().xyz() + other.group1().xyz()).with_w(other[e4]),
            // e5
            self[e5] + other[e5],
            // e41, e42, e43, e45
            other.group3(),
            // e15, e25, e35
            other.group4(),
            // e23, e31, e12
            other.group5(),
            // e415, e425, e435, e321
            self.group0() + other.group6(),
            // e423, e431, e412
            other.group7(),
            // e235, e315, e125
            self.group1() + other.group8(),
            // e1234, e4235, e4315, e4125
            other.group9(),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<MysteryCircle> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: MysteryCircle) -> Self::Output {
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() + other.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<MysteryCircle> for AntiDipoleInversionAtInfinity {
    fn add_assign(&mut self, other: MysteryCircle) {
        *self = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() + other.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Add<MysteryCircleRotor> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([other[e12345], self[e1], self[e2], self[e3]]),
            // e415, e425, e435, e321
            self.group0() + other.group0(),
            // e235, e315, e125, e5
            self.group1().with_w(self[e5]),
        );
    }
}
impl std::ops::Add<MysteryDipole> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MysteryDipoleInversion> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MysteryVersorEven> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: MysteryVersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([0.0, self[e1], other[e2], other[e3]]) + other.group0().xy().with_zw(self[e2], self[e3]),
            // e415, e425, e435, e321
            self.group0() + other.group1(),
            // e235, e315, e125, e5
            self.group1().with_w(self[e5]),
        );
    }
}
impl std::ops::Add<MysteryVersorOdd> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullCircleAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    fn add(self, other: NullCircleAtOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0(),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            self.group1().with_w(0.0),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Add<NullDipoleAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: NullDipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullDipoleInversionAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().xyz().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullSphereAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullVersorEvenAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    fn add(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            self.group1().with_w(other[e4]),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Add<Origin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            self.group1().with_w(other[e4]),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Add<Plane> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<PlaneOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<RoundPoint> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            self.group1().with_w(other[e4]),
            // e1, e2, e3, e5
            self.group2() + other.group0().xyz().with_w(other[e5]),
        );
    }
}
impl std::ops::Add<RoundPointAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            self.group1().with_w(other[e4]),
            // e1, e2, e3, e5
            self.group2().xyz().with_w(self[e5] + other[e5]),
        );
    }
}
impl std::ops::Add<Scalar> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Sphere> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<SphereAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<SphereOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            other.group0().wxyz(),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<VersorEven> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       11        0        0
    fn add(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0(),
            // e415, e425, e435, e321
            self.group0() + other.group1(),
            // e235, e315, e125, e5
            other.group2() + self.group1().with_w(self[e5]),
            // e1, e2, e3, e4
            (self.group2().xyz() + other.group3().xyz()).with_w(other[e4]),
        );
    }
}
impl std::ops::Add<VersorEvenAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0(),
            // e415, e425, e435, e321
            self.group0() + other.group1().xyz().with_w(0.0),
            // e235, e315, e125, e5
            other.group2() + self.group1().with_w(self[e5]),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(other[e4]),
        );
    }
}
impl std::ops::Add<VersorEvenAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        3        0        0
    // no simd       12        0        0
    fn add(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([0.0, self[e1], other[e2], other[e3]]) + other.group0().xy().with_zw(self[e2], self[e3]),
            // e415, e425, e435, e321
            self.group0() + other.group1(),
            // e235, e315, e125, e5
            other.group2() + self.group1().with_w(self[e5]),
        );
    }
}
impl std::ops::Add<VersorEvenAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            (self.group1() + other.group1().xyz()).with_w(other[e4]),
            // e1, e2, e3, e5
            self.group2() + Simd32x3::from(0.0).with_w(other[e5]),
        );
    }
}
impl std::ops::Add<VersorEvenOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: VersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0(),
            // e415, e425, e435, e321
            self.group0() + other.group1().xyz().with_w(0.0),
            // e235, e315, e125, e5
            self.group1().with_w(self[e5]),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(other[e4]),
        );
    }
}
impl std::ops::Add<VersorEvenOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       11        0        0
    fn add(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321]),
            // e235, e315, e125, e4
            (self.group1() + other.group1().xyz()).with_w(other[e4]),
            // e1, e2, e3, e5
            self.group2() + other.group2().xyz().with_w(other[e5]),
        );
    }
}
impl std::ops::Add<VersorOdd> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], other[e45]]),
            // e15, e25, e35
            other.group2().xyz(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<VersorOddAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            other.group0().yzw(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<VersorOddOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().xyz().with_w(0.0),
            // e15, e25, e35
            other.group2().xyz(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            other[e3215],
        );
    }
}

impl From<AntiFlatOrigin> for AntiDipoleInversionAtInfinity {
    fn from(from_anti_flat_origin: AntiFlatOrigin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flat_origin[e321]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiFlatPoint> for AntiDipoleInversionAtInfinity {
    fn from(from_anti_flat_point: AntiFlatPoint) -> Self {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flat_point[e321]),
            // e235, e315, e125
            from_anti_flat_point.group0().xyz(),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiFlector> for AntiDipoleInversionAtInfinity {
    fn from(from_anti_flector: AntiFlector) -> Self {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flector[e321]),
            // e235, e315, e125
            from_anti_flector.group0().xyz(),
            // e1, e2, e3, e5
            from_anti_flector.group1(),
        );
    }
}

impl From<AntiFlectorOnOrigin> for AntiDipoleInversionAtInfinity {
    fn from(from_anti_flector_on_origin: AntiFlectorOnOrigin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flector_on_origin[e321]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([from_anti_flector_on_origin[e1], from_anti_flector_on_origin[e2], from_anti_flector_on_origin[e3], 0.0]),
        );
    }
}

impl From<AntiMysteryDipoleInversion> for AntiDipoleInversionAtInfinity {
    fn from(from_anti_mystery_dipole_inversion: AntiMysteryDipoleInversion) -> Self {
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            from_anti_mystery_dipole_inversion.group0(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            from_anti_mystery_dipole_inversion.group1().with_w(0.0),
        );
    }
}

impl From<AntiPlane> for AntiDipoleInversionAtInfinity {
    fn from(from_anti_plane: AntiPlane) -> Self {
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            from_anti_plane.group0(),
        );
    }
}

impl From<AntiPlaneOnOrigin> for AntiDipoleInversionAtInfinity {
    fn from(from_anti_plane_on_origin: AntiPlaneOnOrigin) -> Self {
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            from_anti_plane_on_origin.group0().with_w(0.0),
        );
    }
}

impl From<CircleAtInfinity> for AntiDipoleInversionAtInfinity {
    fn from(from_circle_at_infinity: CircleAtInfinity) -> Self {
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            from_circle_at_infinity.group0(),
            // e235, e315, e125
            from_circle_at_infinity.group1(),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<Infinity> for AntiDipoleInversionAtInfinity {
    fn from(from_infinity: Infinity) -> Self {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(from_infinity[e5]),
        );
    }
}

impl From<Line> for AntiDipoleInversionAtInfinity {
    fn from(from_line: Line) -> Self {
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            from_line.group0().with_w(0.0),
            // e235, e315, e125
            from_line.group1(),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<LineAtInfinity> for AntiDipoleInversionAtInfinity {
    fn from(from_line_at_infinity: LineAtInfinity) -> Self {
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125
            from_line_at_infinity.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<LineOnOrigin> for AntiDipoleInversionAtInfinity {
    fn from(from_line_on_origin: LineOnOrigin) -> Self {
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            from_line_on_origin.group0().with_w(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<MotorAtInfinity> for AntiDipoleInversionAtInfinity {
    fn from(from_motor_at_infinity: MotorAtInfinity) -> Self {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125
            from_motor_at_infinity.group0().xyz(),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(from_motor_at_infinity[e5]),
        );
    }
}

impl From<MysteryCircle> for AntiDipoleInversionAtInfinity {
    fn from(from_mystery_circle: MysteryCircle) -> Self {
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            from_mystery_circle.group0(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Mul<AntiCircleOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       42        0
    //    simd3        0        7        0
    //    simd4       11        4        0
    // Totals...
    // yes simd       32       53        0
    //  no simd       65       79        0
    fn mul(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       39        0
    //    simd3        0        6        0
    //    simd4       21       16        0
    // Totals...
    // yes simd       42       61        0
    //  no simd      105      121        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       43        0
    //    simd3        0        8        0
    //    simd4       19       11        0
    // Totals...
    // yes simd       40       62        0
    //  no simd       97      111        0
    fn mul(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOriginAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       24        0
    //    simd3        0        3        0
    //    simd4       11        8        0
    // Totals...
    // yes simd       20       35        0
    //  no simd       53       65        0
    fn mul(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       21        0
    //    simd4       14       14        0
    // Totals...
    // yes simd       22       35        0
    //  no simd       64       77        0
    fn mul(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       46        0
    //    simd3        0        8        0
    //    simd4       13        5        0
    // Totals...
    // yes simd       37       59        0
    //  no simd       76       90        0
    fn mul(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       63        0
    //    simd3        0       15        0
    //    simd4       29       15        0
    // Totals...
    // yes simd       63       93        0
    //  no simd      150      168        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       39        0
    //    simd3        0        2        0
    //    simd4       17       15        0
    // Totals...
    // yes simd       46       56        0
    //  no simd       97      105        0
    fn mul(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       43        0
    //    simd3        0        3        0
    //    simd4       16       13        0
    // Totals...
    // yes simd       41       59        0
    //  no simd       89      104        0
    fn mul(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       46        0
    //    simd3        0        6        0
    //    simd4       21       15        0
    // Totals...
    // yes simd       45       67        0
    //  no simd      108      124        0
    fn mul(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       26        0
    //    simd3        3        6        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       22       36        0
    //  no simd       40       60        0
    fn mul(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        7        0
    //    simd3        2        4        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3       13        0
    //  no simd       10       27        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       22        0
    fn mul(self, other: AntiFlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       26        0
    //    simd3        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       21       30        0
    //  no simd       21       39        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       35        0
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4       10        8        0
    // Totals...
    // yes simd       35       45        0
    //  no simd       65       72        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlectorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       12        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       12       20        0
    //  no simd       36       44        0
    fn mul(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       21        0
    //    simd3        0        3        0
    //    simd4        9        6        0
    // Totals...
    // yes simd       15       30        0
    //  no simd       42       54        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLineOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
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
impl std::ops::Mul<AntiMotor> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       29        0
    //    simd3        0        5        0
    //    simd4       12        7        0
    // Totals...
    // yes simd       24       41        0
    //  no simd       60       72        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
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
impl std::ops::Mul<AntiMysteryCircleRotor> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       24        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       19       32        0
    //  no simd       43       56        0
    fn mul(self, other: AntiMysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryDipoleInversion> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       36        0
    //    simd3        0        4        0
    //    simd4       12        8        0
    // Totals...
    // yes simd       29       48        0
    //  no simd       65       80        0
    fn mul(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       24        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       13       29        0
    //  no simd       28       44        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlaneOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       22        0
    //    simd3        0        1        0
    //    simd4        4        3        0
    // Totals...
    // yes simd        9       26        0
    //  no simd       21       37        0
    fn mul(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for AntiDipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
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
impl std::ops::Mul<AntiSphereOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       24        0
    //    simd3        2        3        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       13       32        0
    //  no simd       32       53        0
    fn mul(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorEvenOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       43        0
    //    simd3        0        3        0
    //    simd4       16       13        0
    // Totals...
    // yes simd       40       59        0
    //  no simd       88      104        0
    fn mul(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       49        0
    //    simd3        0       12        0
    //    simd4       19        7        0
    // Totals...
    // yes simd       40       68        0
    //  no simd       97      113        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       40        0
    //    simd3        0        9        0
    //    simd4       17        8        0
    // Totals...
    // yes simd       38       57        0
    //  no simd       89       99        0
    fn mul(self, other: CircleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       27        0
    //    simd3        0        1        0
    //    simd4       10        9        0
    // Totals...
    // yes simd       27       37        0
    //  no simd       57       66        0
    fn mul(self, other: CircleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       35        0
    //    simd3        0        5        0
    //    simd4        9        4        0
    // Totals...
    // yes simd       26       44        0
    //  no simd       53       66        0
    fn mul(self, other: CircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       42        0
    //    simd3        0        8        0
    //    simd4       11        3        0
    // Totals...
    // yes simd       33       53        0
    //  no simd       66       78        0
    fn mul(self, other: CircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       37        0
    //    simd3        0        4        0
    //    simd4       11        7        0
    // Totals...
    // yes simd       31       48        0
    //  no simd       64       77        0
    fn mul(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       55        0
    //    simd3        0       14        0
    //    simd4       20        6        0
    // Totals...
    // yes simd       48       75        0
    //  no simd      108      121        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       55        0
    //    simd3        0       14        0
    //    simd4       18        4        0
    // Totals...
    // yes simd       43       73        0
    //  no simd       97      113        0
    fn mul(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOriginAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       31        0
    //    simd3        0        5        0
    //    simd4       10        5        0
    // Totals...
    // yes simd       30       41        0
    //  no simd       60       66        0
    fn mul(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       24        0
    //    simd4       12       13        0
    // Totals...
    // yes simd       32       37        0
    //  no simd       68       76        0
    fn mul(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       50        0
    //    simd3        0        6        0
    //    simd4       12        6        0
    // Totals...
    // yes simd       38       62        0
    //  no simd       74       92        0
    fn mul(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       40        0
    //    simd3        0       10        0
    //    simd4       19       10        0
    // Totals...
    // yes simd       37       60        0
    //  no simd       94      110        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       26        0
    //    simd3        0        1        0
    //    simd4       13       12        0
    // Totals...
    // yes simd       25       39        0
    //  no simd       64       77        0
    fn mul(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       19        0
    //    simd3        0        1        0
    //    simd4       12       11        0
    // Totals...
    // yes simd       17       31        0
    //  no simd       53       66        0
    fn mul(self, other: DipoleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       31        0
    //    simd3        0        5        0
    //    simd4       11        6        0
    // Totals...
    // yes simd       20       42        0
    //  no simd       53       70        0
    fn mul(self, other: DipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       33        0
    //    simd3        0        9        0
    //    simd4       35       27        0
    // Totals...
    // yes simd       45       69        0
    //  no simd      150      168        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       34        0
    //    simd3        0        5        0
    //    simd4       26       21        0
    // Totals...
    // yes simd       38       60        0
    //  no simd      116      133        0
    fn mul(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1       15        0
    //    simd3        0        3        0
    //    simd4       23       21        0
    // Totals...
    // yes simd       24       39        0
    //  no simd       93      108        0
    fn mul(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       19        0
    //    simd3        0        2        0
    //    simd4       16       16        0
    // Totals...
    // yes simd       27       37        0
    //  no simd       75       89        0
    fn mul(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       34        0
    //    simd3        0        5        0
    //    simd4       19       14        0
    // Totals...
    // yes simd       31       53        0
    //  no simd       88      105        0
    fn mul(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       47        0
    //    simd3        0       10        0
    //    simd4       21       11        0
    // Totals...
    // yes simd       42       68        0
    //  no simd      105      121        0
    fn mul(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       19       33        0
    //  no simd       40       60        0
    fn mul(self, other: DipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       43        0
    //    simd3        0       11        0
    //    simd4       17        6        0
    // Totals...
    // yes simd       35       60        0
    //  no simd       86      100        0
    fn mul(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       11        0
    //    simd3        2        4        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        3       18        0
    //  no simd       10       35        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       22        0
    fn mul(self, other: FlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        6        8        0
    // Totals...
    // yes simd        6       11        0
    //  no simd       24       39        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPointAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        5        5        0
    // Totals...
    // yes simd        5        6        0
    //  no simd       20       21        0
    fn mul(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       27        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       24       39        0
    //  no simd       60       75        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        6        7        0
    // no simd       24       28        0
    fn mul(self, other: FlectorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       27        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       17       32        0
    //  no simd       32       47        0
    fn mul(self, other: FlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for AntiDipoleInversionAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        3        8        0
    fn mul(self, other: Horizon) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Infinity> for AntiDipoleInversionAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        3        7        0
    fn mul(self, other: Infinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       27        0
    //    simd3        0        1        0
    //    simd4        7        6        0
    // Totals...
    // yes simd       25       34        0
    //  no simd       46       54        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        5        5        0
    // Totals...
    // yes simd        5        7        0
    //  no simd       20       22        0
    fn mul(self, other: LineAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       21        0
    //    simd3        0        1        0
    //    simd4        4        3        0
    // Totals...
    // yes simd        9       25        0
    //  no simd       21       36        0
    fn mul(self, other: LineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       33        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       37       43        0
    //  no simd       67       73        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        6        7        0
    // no simd       24       28        0
    fn mul(self, other: MotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       14       21        0
    //  no simd       35       45        0
    fn mul(self, other: MotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       64       92        0
    //    simd2        4        4        0
    //    simd3       44       57        0
    //    simd4       30       21        0
    // Totals...
    // yes simd      142      174        0
    //  no simd      324      355        0
    fn mul(self, other: MultiVector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircle> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       13        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       11       21        0
    //  no simd       32       45        0
    fn mul(self, other: MysteryCircle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircleRotor> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       16        0
    //    simd4        9       10        0
    // Totals...
    // yes simd       16       26        0
    //  no simd       43       56        0
    fn mul(self, other: MysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipole> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       21        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       14       27        0
    //  no simd       32       45        0
    fn mul(self, other: MysteryDipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipoleInversion> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1       16        0
    //    simd3        0        4        0
    //    simd4       16       13        0
    // Totals...
    // yes simd       17       33        0
    //  no simd       65       80        0
    fn mul(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorEven> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       26        0
    //    simd3        0        1        0
    //    simd4       16       15        0
    // Totals...
    // yes simd       31       42        0
    //  no simd       79       89        0
    fn mul(self, other: MysteryVersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorOdd> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       15        0
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4       19       18        0
    // Totals...
    // yes simd       19       35        0
    //  no simd       76       92        0
    fn mul(self, other: MysteryVersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullCircleAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       27        0
    //    simd3        0        6        0
    //    simd4        5        0        0
    // Totals...
    // yes simd       20       33        0
    //  no simd       35       45        0
    fn mul(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       30        0
    //    simd3        0        5        0
    //    simd4        6        1        0
    // Totals...
    // yes simd       15       36        0
    //  no simd       33       49        0
    fn mul(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleInversionAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       17        0
    //    simd4       10       11        0
    // Totals...
    // yes simd       21       28        0
    //  no simd       51       61        0
    fn mul(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullSphereAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1        7        0
    //  no simd        3       23        0
    fn mul(self, other: NullSphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullVersorEvenAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       28        0
    //    simd3        0        4        0
    //    simd4        8        5        0
    // Totals...
    // yes simd       27       37        0
    //  no simd       51       60        0
    fn mul(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Origin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        1        4        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1       10        0
    //  no simd        3       27        0
    fn mul(self, other: Origin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       23        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       13       28        0
    //  no simd       28       43        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<PlaneOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       26        0
    //    simd3        0        2        0
    //    simd4        3        1        0
    // Totals...
    // yes simd       12       29        0
    //  no simd       21       36        0
    fn mul(self, other: PlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       25        0
    //    simd3        4        5        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       16       35        0
    //  no simd       39       60        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPointAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        2        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2       12        0
    //  no simd        6       32        0
    fn mul(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
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
impl std::ops::MulAssign<Scalar> for AntiDipoleInversionAtInfinity {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       20        0
    //    simd3        2        6        0
    //    simd4        8        5        0
    // Totals...
    // yes simd       14       31        0
    //  no simd       42       58        0
    fn mul(self, other: Sphere) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        2        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2       11        0
    //  no simd        6       31        0
    fn mul(self, other: SphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       18        0
    //    simd3        2        3        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       13       27        0
    //  no simd       35       51        0
    fn mul(self, other: SphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       60        0
    //    simd3        0        9        0
    //    simd4       32       23        0
    // Totals...
    // yes simd       68       92        0
    //  no simd      164      179        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       46        0
    //    simd3        0        3        0
    //    simd4       23       20        0
    // Totals...
    // yes simd       50       69        0
    //  no simd      119      135        0
    fn mul(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       39        0
    //    simd3        0        2        0
    //    simd4       20       18        0
    // Totals...
    // yes simd       48       59        0
    //  no simd      108      117        0
    fn mul(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       36        0
    //    simd3        0        4        0
    //    simd4       14       10        0
    // Totals...
    // yes simd       33       50        0
    //  no simd       75       88        0
    fn mul(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       50        0
    //    simd3        0        3        0
    //    simd4       15       12        0
    // Totals...
    // yes simd       44       65        0
    //  no simd       89      107        0
    fn mul(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       44        0
    //    simd3        0        4        0
    //    simd4       23       19        0
    // Totals...
    // yes simd       50       67        0
    //  no simd      119      132        0
    fn mul(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       32        0
    //    simd3        0        5        0
    //    simd4       37       33        0
    // Totals...
    // yes simd       50       70        0
    //  no simd      161      179        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       13        0
    //    simd3        0        1        0
    //    simd4       26       26        0
    // Totals...
    // yes simd       26       40        0
    //  no simd      104      120        0
    fn mul(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       45        0
    //    simd3        0        5        0
    //    simd4       23       18        0
    // Totals...
    // yes simd       47       68        0
    //  no simd      116      132        0
    fn mul(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Neg for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn neg(self) -> Self::Output {
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
            // e1, e2, e3, e5
            self.group2() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Not for AntiDipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiCircleOnOrigin> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotor> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(other[e45]) * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOrigin> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOriginAtInfinity> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAtInfinity> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorOnOrigin> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiDipoleInversion> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        3        2        0
    //  no simd       11        7        0
    fn sub(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0() - other.group1(),
            // e235, e315, e125, e4
            (self.group1() - other.group2().xyz()).with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1, e2, e3, e5
            self.group2() - other.group3(),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       11        0        0
    fn sub(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() - other.group0(),
            // e235, e315, e125
            self.group1() - other.group1(),
            // e1, e2, e3, e5
            self.group2() - other.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiDipoleInversionAtInfinity> for AntiDipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: AntiDipoleInversionAtInfinity) {
        *self = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() - other.group0(),
            // e235, e315, e125
            self.group1() - other.group1(),
            // e1, e2, e3, e5
            self.group2() - other.group2(),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        8        8        0
    fn sub(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125, e4
            self.group1().with_w(other[e4] * -1.0),
            // e1, e2, e3, e5
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        1        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        3        6        0
    //  no simd       11       11        0
    fn sub(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group0(),
            // e235, e315, e125, e4
            (self.group1() - other.group2().xyz()).with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1, e2, e3, e5
            self.group2() + Simd32x3::from(0.0).with_w(other[e5] * -1.0),
        );
    }
}
impl std::ops::Sub<AntiDipoleOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        4        4        0
    fn sub(self, other: AntiDipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125, e4
            self.group1().with_w(0.0),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiDualNum> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiFlatOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiFlatOrigin> for AntiDipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: AntiFlatOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiFlatPoint> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        7        1        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125
            self.group1() - other.group0().xyz(),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiFlatPoint> for AntiDipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: AntiFlatPoint) {
        use crate::elements::*;
        *self = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125
            self.group1() - other.group0().xyz(),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiFlector> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        1        0
    //  no simd       11        1        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125
            self.group1() - other.group0().xyz(),
            // e1, e2, e3, e5
            self.group2() - other.group1(),
        );
    }
}
impl std::ops::SubAssign<AntiFlector> for AntiDipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: AntiFlector) {
        use crate::elements::*;
        *self = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125
            self.group1() - other.group0().xyz(),
            // e1, e2, e3, e5
            self.group2() - other.group1(),
        );
    }
}
impl std::ops::Sub<AntiFlectorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        4        0
    fn sub(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiFlectorOnOrigin> for AntiDipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: AntiFlectorOnOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<AntiLine> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiLineOnOrigin> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiMotor> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<AntiMotorOnOrigin> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiMysteryCircleRotor> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiMysteryDipoleInversion> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        8        3        0
    fn sub(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() - other.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiMysteryDipoleInversion> for AntiDipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: AntiMysteryDipoleInversion) {
        use crate::elements::*;
        *self = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() - other.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<AntiPlane> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            self.group2() - other.group0(),
        );
    }
}
impl std::ops::SubAssign<AntiPlane> for AntiDipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: AntiPlane) {
        *self = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            self.group2() - other.group0(),
        );
    }
}
impl std::ops::Sub<AntiPlaneOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        4        3        0
    fn sub(self, other: AntiPlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiPlaneOnOrigin> for AntiDipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: AntiPlaneOnOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<AntiScalar> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([other[e12345], self[e1], self[e2], self[e3]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1().with_w(self[e5]),
        );
    }
}
impl std::ops::Sub<AntiSphereOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        4        4        0
    fn sub(self, other: AntiSphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            self.group1().with_w(other[e4] * -1.0),
            // e1, e2, e3, e5
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group2(),
        );
    }
}
impl std::ops::Sub<AntiVersorEvenOnOrigin> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Circle> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        7        3        0
    fn sub(self, other: Circle) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0() - other.group1(),
            // e235, e315, e125, e4
            (self.group1() - other.group2()).with_w(0.0),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Sub<CircleAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        1        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        7        6        0
    fn sub(self, other: CircleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group0(),
            // e235, e315, e125, e4
            (self.group1() - other.group2()).with_w(0.0),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Sub<CircleAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn sub(self, other: CircleAtInfinity) -> Self::Output {
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() - other.group0(),
            // e235, e315, e125
            self.group1() - other.group1(),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<CircleAtInfinity> for AntiDipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: CircleAtInfinity) {
        *self = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() - other.group0(),
            // e235, e315, e125
            self.group1() - other.group1(),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Sub<CircleAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        1        0
    // no simd        3        3        0
    fn sub(self, other: CircleAtOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            (self.group1() - other.group1()).with_w(0.0),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Sub<CircleOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        4        6        0
    fn sub(self, other: CircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group0(),
            // e235, e315, e125, e4
            self.group1().with_w(0.0),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Sub<CircleOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        7        4        0
    fn sub(self, other: CircleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125, e4
            (self.group1() - other.group1()).with_w(0.0),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Sub<CircleRotor> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        7        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0().with_w(other[e12345]) * Simd32x4::from(-1.0),
            // e415, e425, e435, e321
            self.group0() - other.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group1().with_w(self[e5]),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        8       10        0
    fn sub(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0().with_w(other[e12345]) * Simd32x4::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group1().with_w(self[e5]),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOriginAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        8       10        0
    fn sub(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([other[e12345], self[e1], self[e2], self[e3]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group1().with_w(self[e5]),
        );
    }
}
impl std::ops::Sub<CircleRotorAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        7        0
    fn sub(self, other: CircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([other[e12345], self[e1], self[e2], self[e3]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e415, e425, e435, e321
            self.group0() - other.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0, 0.0]) + self.group1().with_w(self[e5]),
        );
    }
}
impl std::ops::Sub<CircleRotorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        4        7        0
    fn sub(self, other: CircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group0(),
            // e235, e315, e125, e5
            self.group1().with_w(self[e5]),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(0.0),
        );
    }
}
impl std::ops::Sub<Dipole> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(other[e45]) * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group2() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleAligningOrigin> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleAtInfinity> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleAtOrigin> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleInversion> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().with_w(other[e45]) * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionAligningOrigin> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionAtInfinity> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionAtOrigin> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionOnOrigin> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            other.group1() * Simd32x4::from(-1.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleOnOrigin> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            other.group2() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DualNum> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(other[e12345]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1().with_w(self[e5]),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<FlatOrigin> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<FlatPoint> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<FlatPointAtInfinity> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Flector> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<FlectorAtInfinity> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<FlectorOnOrigin> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Horizon> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<Infinity> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            self.group2() + Simd32x3::from(0.0).with_w(other[e5] * -1.0),
        );
    }
}
impl std::ops::SubAssign<Infinity> for AntiDipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: Infinity) {
        use crate::elements::*;
        *self = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            self.group2() + Simd32x3::from(0.0).with_w(other[e5] * -1.0),
        );
    }
}
impl std::ops::Sub<Line> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
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
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group0(),
            // e235, e315, e125
            self.group1() - other.group1(),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<Line> for AntiDipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: Line) {
        use crate::elements::*;
        *self = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group0(),
            // e235, e315, e125
            self.group1() - other.group1(),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Sub<LineAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn sub(self, other: LineAtInfinity) -> Self::Output {
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1() - other.group0(),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<LineAtInfinity> for AntiDipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: LineAtInfinity) {
        *self = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1() - other.group0(),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Sub<LineOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        4        3        0
    fn sub(self, other: LineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<LineOnOrigin> for AntiDipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: LineOnOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Sub<Motor> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        7        0
    fn sub(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([other[e12345], self[e1], self[e2], self[e3]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group0(),
            // e235, e315, e125, e5
            self.group1().with_w(self[e5]) - other.group1(),
        );
    }
}
impl std::ops::Sub<MotorAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        7        1        0
    fn sub(self, other: MotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1() - other.group0().xyz(),
            // e1, e2, e3, e5
            self.group2() + Simd32x3::from(0.0).with_w(other[e5] * -1.0),
        );
    }
}
impl std::ops::SubAssign<MotorAtInfinity> for AntiDipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: MotorAtInfinity) {
        use crate::elements::*;
        *self = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1() - other.group0().xyz(),
            // e1, e2, e3, e5
            self.group2() + Simd32x3::from(0.0).with_w(other[e5] * -1.0),
        );
    }
}
impl std::ops::Sub<MotorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        4        7        0
    fn sub(self, other: MotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([other[e12345], self[e1], self[e2], self[e3]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group0(),
            // e235, e315, e125, e5
            self.group1().with_w(self[e5]),
        );
    }
}
impl std::ops::Sub<MultiVector> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        1        0
    //    simd2        0        1        0
    //    simd3        2        3        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        4        8        0
    //  no simd       11       24        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            other.group0() * Simd32x2::from(-1.0),
            // e1, e2, e3, e4
            (self.group2().xyz() - other.group1().xyz()).with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            other.group3() * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group4() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group5() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0() - other.group6(),
            // e423, e431, e412
            other.group7() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() - other.group8(),
            // e1234, e4235, e4315, e4125
            other.group9() * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<MysteryCircle> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: MysteryCircle) -> Self::Output {
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() - other.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<MysteryCircle> for AntiDipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: MysteryCircle) {
        *self = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() - other.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Sub<MysteryCircleRotor> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        1        0
    // no simd        4        4        0
    fn sub(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([other[e12345], self[e1], self[e2], self[e3]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e415, e425, e435, e321
            self.group0() - other.group0(),
            // e235, e315, e125, e5
            self.group1().with_w(self[e5]),
        );
    }
}
impl std::ops::Sub<MysteryDipole> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MysteryDipoleInversion> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MysteryVersorEven> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        7        4        0
    fn sub(self, other: MysteryVersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([other[e12345], self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e415, e425, e435, e321
            self.group0() - other.group1(),
            // e235, e315, e125, e5
            self.group1().with_w(self[e5]),
        );
    }
}
impl std::ops::Sub<MysteryVersorOdd> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullCircleAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: NullCircleAtOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            self.group1().with_w(0.0),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Sub<NullDipoleAtOrigin> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullDipoleInversionAtOrigin> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullSphereAtOrigin> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullVersorEvenAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            self.group1().with_w(other[e4] * -1.0),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Sub<Origin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            self.group1().with_w(other[e4] * -1.0),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Sub<Plane> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<PlaneOnOrigin> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<RoundPoint> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            self.group1().with_w(other[e4] * -1.0),
            // e1, e2, e3, e5
            self.group2() - other.group0().xyz().with_w(other[e5]),
        );
    }
}
impl std::ops::Sub<RoundPointAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        4        2        0
    fn sub(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            self.group1().with_w(other[e4] * -1.0),
            // e1, e2, e3, e5
            self.group2() + Simd32x3::from(0.0).with_w(other[e5] * -1.0),
        );
    }
}
impl std::ops::Sub<Scalar> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Sphere> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<SphereAtOrigin> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<SphereOnOrigin> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            other.group0().wxyz() * Simd32x4::from(-1.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<VersorEven> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        3        2        0
    //  no simd       11        8        0
    fn sub(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435, e321
            self.group0() - other.group1(),
            // e235, e315, e125, e5
            self.group1().with_w(self[e5]) - other.group2(),
            // e1, e2, e3, e4
            (self.group2().xyz() - other.group3().xyz()).with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<VersorEvenAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        8       11        0
    fn sub(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group0(),
            // e235, e315, e125, e5
            self.group1().with_w(self[e5]) - other.group2(),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<VersorEvenAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        5        1        0
    //  no simd       11        4        0
    fn sub(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([other[e12345], self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e415, e425, e435, e321
            self.group0() - other.group1(),
            // e235, e315, e125, e5
            self.group1().with_w(self[e5]) - other.group2(),
        );
    }
}
impl std::ops::Sub<VersorEvenAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        7        8        0
    fn sub(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            (self.group1() - other.group1().xyz()).with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1, e2, e3, e5
            self.group2() + Simd32x3::from(0.0).with_w(other[e5] * -1.0),
        );
    }
}
impl std::ops::Sub<VersorEvenOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        1        5        0
    //  no simd        4       11        0
    fn sub(self, other: VersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group0(),
            // e235, e315, e125, e5
            self.group1().with_w(self[e5]),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<VersorEvenOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        3        3        0
    //  no simd       11        8        0
    fn sub(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e235, e315, e125, e4
            (self.group1() - other.group1().xyz()).with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1, e2, e3, e5
            self.group2() - other.group2().xyz().with_w(other[e5]),
        );
    }
}
impl std::ops::Sub<VersorOdd> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0().xyz().with_w(other[e45]) * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<VersorOddAtInfinity> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            other.group0().yzw() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<VersorOddOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
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
            self.group2().xyz().with_w(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            (Simd32x2::from([other[e1234], 0.0]) * Simd32x2::from([-1.0, 0.0])).with_zw(0.0, 0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}

impl TryFrom<AntiDipoleInversion> for AntiDipoleInversionAtInfinity {
    type Error = String;
    fn try_from(anti_dipole_inversion: AntiDipoleInversion) -> Result<Self, Self::Error> {
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
        let el = anti_dipole_inversion[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversion do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            anti_dipole_inversion.group1(),
            // e235, e315, e125
            anti_dipole_inversion.group2().xyz(),
            // e1, e2, e3, e5
            anti_dipole_inversion.group3(),
        ));
    }
}

impl TryFrom<AntiDipoleInversionOnOrigin> for AntiDipoleInversionAtInfinity {
    type Error = String;
    fn try_from(anti_dipole_inversion_on_origin: AntiDipoleInversionOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_on_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversionOnOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(anti_dipole_inversion_on_origin[e321]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([anti_dipole_inversion_on_origin[e1], anti_dipole_inversion_on_origin[e2], anti_dipole_inversion_on_origin[e3], 0.0]),
        ));
    }
}

impl TryFrom<AntiDipoleInversionOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Error = String;
    fn try_from(anti_dipole_inversion_orthogonal_origin: AntiDipoleInversionOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion_orthogonal_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_orthogonal_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_orthogonal_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_orthogonal_origin[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversionOrthogonalOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            anti_dipole_inversion_orthogonal_origin.group1().with_w(0.0),
            // e235, e315, e125
            anti_dipole_inversion_orthogonal_origin.group2().xyz(),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(anti_dipole_inversion_orthogonal_origin[e5]),
        ));
    }
}

impl TryFrom<AntiDipoleOnOrigin> for AntiDipoleInversionAtInfinity {
    type Error = String;
    fn try_from(anti_dipole_on_origin: AntiDipoleOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleOnOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(anti_dipole_on_origin[e321]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiSphereOnOrigin> for AntiDipoleInversionAtInfinity {
    type Error = String;
    fn try_from(anti_sphere_on_origin: AntiSphereOnOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_sphere_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiSphereOnOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            anti_sphere_on_origin.group0().xyz().with_w(0.0),
        ));
    }
}

impl TryFrom<Circle> for AntiDipoleInversionAtInfinity {
    type Error = String;
    fn try_from(circle: Circle) -> Result<Self, Self::Error> {
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
        if fail {
            let mut error = "Elements from Circle do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            circle.group1(),
            // e235, e315, e125
            circle.group2(),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Error = String;
    fn try_from(circle_aligning_origin: CircleAligningOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_aligning_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_aligning_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_aligning_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleAligningOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            circle_aligning_origin.group1().with_w(0.0),
            // e235, e315, e125
            circle_aligning_origin.group2(),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleAtOrigin> for AntiDipoleInversionAtInfinity {
    type Error = String;
    fn try_from(circle_at_origin: CircleAtOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_at_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_at_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_at_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleAtOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125
            circle_at_origin.group1(),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleOnOrigin> for AntiDipoleInversionAtInfinity {
    type Error = String;
    fn try_from(circle_on_origin: CircleOnOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleOnOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            circle_on_origin.group1().with_w(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Error = String;
    fn try_from(circle_orthogonal_origin: CircleOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_orthogonal_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_orthogonal_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_orthogonal_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleOrthogonalOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(circle_orthogonal_origin[e321]),
            // e235, e315, e125
            circle_orthogonal_origin.group1(),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleRotor> for AntiDipoleInversionAtInfinity {
    type Error = String;
    fn try_from(circle_rotor: CircleRotor) -> Result<Self, Self::Error> {
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
        let el = circle_rotor[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotor do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            circle_rotor.group1(),
            // e235, e315, e125
            circle_rotor.group2().xyz(),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleRotorAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Error = String;
    fn try_from(circle_rotor_aligning_origin: CircleRotorAligningOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor_aligning_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
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
            let mut error = "Elements from CircleRotorAligningOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            circle_rotor_aligning_origin.group1().with_w(0.0),
            // e235, e315, e125
            circle_rotor_aligning_origin.group2().xyz(),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleRotorAligningOriginAtInfinity> for AntiDipoleInversionAtInfinity {
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
            let mut error = "Elements from CircleRotorAligningOriginAtInfinity do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            circle_rotor_aligning_origin_at_infinity.group0().with_w(0.0),
            // e235, e315, e125
            circle_rotor_aligning_origin_at_infinity.group1().xyz(),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleRotorAtInfinity> for AntiDipoleInversionAtInfinity {
    type Error = String;
    fn try_from(circle_rotor_at_infinity: CircleRotorAtInfinity) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor_at_infinity[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotorAtInfinity do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            circle_rotor_at_infinity.group0(),
            // e235, e315, e125
            circle_rotor_at_infinity.group1().xyz(),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleRotorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Error = String;
    fn try_from(circle_rotor_on_origin: CircleRotorOnOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotorOnOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            circle_rotor_on_origin.group1().with_w(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<Motor> for AntiDipoleInversionAtInfinity {
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
            let mut error = "Elements from Motor do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            motor.group0().xyz().with_w(0.0),
            // e235, e315, e125
            motor.group1().xyz(),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(motor[e5]),
        ));
    }
}

impl TryFrom<MotorOnOrigin> for AntiDipoleInversionAtInfinity {
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
            let mut error = "Elements from MotorOnOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            motor_on_origin.group0().xyz().with_w(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<MultiVector> for AntiDipoleInversionAtInfinity {
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
            let mut error = "Elements from MultiVector do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            multi_vector.group6(),
            // e235, e315, e125
            multi_vector.group8(),
            // e1, e2, e3, e5
            Simd32x4::from([multi_vector[e1], multi_vector[e2], multi_vector[e3], multi_vector[e5]]),
        ));
    }
}

impl TryFrom<MysteryCircleRotor> for AntiDipoleInversionAtInfinity {
    type Error = String;
    fn try_from(mystery_circle_rotor: MysteryCircleRotor) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_circle_rotor[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryCircleRotor do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            mystery_circle_rotor.group0(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<MysteryVersorEven> for AntiDipoleInversionAtInfinity {
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
        if fail {
            let mut error = "Elements from MysteryVersorEven do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            mystery_versor_even.group1(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([mystery_versor_even[e1], mystery_versor_even[e2], mystery_versor_even[e3], 0.0]),
        ));
    }
}

impl TryFrom<RoundPoint> for AntiDipoleInversionAtInfinity {
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
            let mut error = "Elements from RoundPoint do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([round_point[e1], round_point[e2], round_point[e3], round_point[e5]]),
        ));
    }
}

impl TryFrom<RoundPointAtOrigin> for AntiDipoleInversionAtInfinity {
    type Error = String;
    fn try_from(round_point_at_origin: RoundPointAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = round_point_at_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from RoundPointAtOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(round_point_at_origin[e5]),
        ));
    }
}

impl TryFrom<VersorEven> for AntiDipoleInversionAtInfinity {
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
        let el = versor_even[15];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEven do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            versor_even.group1(),
            // e235, e315, e125
            versor_even.group2().xyz(),
            // e1, e2, e3, e5
            Simd32x4::from([versor_even[e1], versor_even[e2], versor_even[e3], versor_even[e5]]),
        ));
    }
}

impl TryFrom<VersorEvenAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Error = String;
    fn try_from(versor_even_aligning_origin: VersorEvenAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_aligning_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAligningOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            versor_even_aligning_origin.group1().xyz().with_w(0.0),
            // e235, e315, e125
            versor_even_aligning_origin.group2().xyz(),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(versor_even_aligning_origin[e5]),
        ));
    }
}

impl TryFrom<VersorEvenAtInfinity> for AntiDipoleInversionAtInfinity {
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
        if fail {
            let mut error = "Elements from VersorEvenAtInfinity do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            versor_even_at_infinity.group1(),
            // e235, e315, e125
            versor_even_at_infinity.group2().xyz(),
            // e1, e2, e3, e5
            Simd32x4::from([versor_even_at_infinity[e1], versor_even_at_infinity[e2], versor_even_at_infinity[e3], versor_even_at_infinity[e5]]),
        ));
    }
}

impl TryFrom<VersorEvenAtOrigin> for AntiDipoleInversionAtInfinity {
    type Error = String;
    fn try_from(versor_even_at_origin: VersorEvenAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_at_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAtOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125
            versor_even_at_origin.group1().xyz(),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(versor_even_at_origin[e5]),
        ));
    }
}

impl TryFrom<VersorEvenOnOrigin> for AntiDipoleInversionAtInfinity {
    type Error = String;
    fn try_from(versor_even_on_origin: VersorEvenOnOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_on_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenOnOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            versor_even_on_origin.group1().xyz().with_w(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<VersorEvenOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Error = String;
    fn try_from(versor_even_orthogonal_origin: VersorEvenOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_orthogonal_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenOrthogonalOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(versor_even_orthogonal_origin[e321]),
            // e235, e315, e125
            versor_even_orthogonal_origin.group1().xyz(),
            // e1, e2, e3, e5
            Simd32x4::from([
                versor_even_orthogonal_origin[e1],
                versor_even_orthogonal_origin[e2],
                versor_even_orthogonal_origin[e3],
                versor_even_orthogonal_origin[e5],
            ]),
        ));
    }
}
