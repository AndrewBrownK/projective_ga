use crate::traits::GeometricProduct;
use crate::traits::RightDual;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 333
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         1       2       0
//  Average:        11      17       0
//  Maximum:       181     219       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         4       3       0
//  Average:        27      31       0
//  Maximum:       448     480       0
impl std::ops::Add<AntiCircleOnOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: AntiCircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0() + self.group0(),
            // e23, e31, e12, e45
            (other.group1() + self.group1().xyz()).with_w(self[e45]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<AntiCircleOnOrigin> for DipoleInversion {
    fn add_assign(&mut self, other: AntiCircleOnOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0() + self.group0(),
            // e23, e31, e12, e45
            (other.group1() + self.group1().xyz()).with_w(self[e45]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<AntiCircleRotor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (other.group0() + self.group0()).with_w(other[scalar]),
            // e23, e31, e12, e45
            other.group1() + self.group1(),
            // e15, e25, e35, e1234
            (other.group2().xyz() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        3        0        0
    // no simd        9        0        0
    fn add(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (other.group0() + self.group0()).with_w(other[scalar]),
            // e23, e31, e12, e45
            (other.group1() + self.group1().xyz()).with_w(self[e45]),
            // e15, e25, e35, e1234
            (other.group2().xyz() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOriginAtInfinity> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(other[scalar]),
            // e23, e31, e12, e45
            (other.group0() + self.group1().xyz()).with_w(self[e45]),
            // e15, e25, e35, e1234
            (other.group1().xyz() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<AntiCircleRotorAtInfinity> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(other[scalar]),
            // e23, e31, e12, e45
            other.group0() + self.group1(),
            // e15, e25, e35, e1234
            (other.group1().xyz() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<AntiCircleRotorOnOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0() + other.group0().xyz()).with_w(other[scalar]),
            // e23, e31, e12, e45
            (other.group1() + self.group1().xyz()).with_w(self[e45]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<AntiDipoleInversion> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], other[e4]]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDipoleInversionAtInfinity> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group2().xyz().with_w(0.0),
            // e5
            other[e5],
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOnOrigin> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group1().yzwx(),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOrthogonalOrigin> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDipoleOnOrigin> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDualNum> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(other[scalar]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(other[e1234] + self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<AntiFlatOrigin> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiFlatPoint> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiFlector> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group1().xyz().with_w(0.0),
            // e5
            other[e5],
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiFlectorOnOrigin> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiLine> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            (other.group0() + self.group1().xyz()).with_w(self[e45]),
            // e15, e25, e35, e1234
            (other.group1() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<AntiLine> for DipoleInversion {
    fn add_assign(&mut self, other: AntiLine) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            (other.group0() + self.group1().xyz()).with_w(self[e45]),
            // e15, e25, e35, e1234
            (other.group1() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<AntiLineOnOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: AntiLineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            (other.group0() + self.group1().xyz()).with_w(self[e45]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<AntiLineOnOrigin> for DipoleInversion {
    fn add_assign(&mut self, other: AntiLineOnOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            (other.group0() + self.group1().xyz()).with_w(self[e45]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<AntiMotor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(other[scalar]),
            // e23, e31, e12, e45
            (other.group0().xyz() + self.group1().xyz()).with_w(self[e45]),
            // e15, e25, e35, e1234
            (other.group1().xyz() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3().xyz().with_w(other[e3215] + self[e3215]),
        );
    }
}
impl std::ops::Add<AntiMotorOnOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(other[scalar]),
            // e23, e31, e12, e45
            (other.group0().xyz() + self.group1().xyz()).with_w(self[e45]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<AntiMysteryCircleRotor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(other[scalar]),
            // e23, e31, e12, e45
            other.group0() + self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<AntiMysteryDipoleInversion> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group1().with_w(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiPlane> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group0().xyz().with_w(0.0),
            // e5
            other[e5],
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiPlaneOnOrigin> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiPlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group0().with_w(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiScalar> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiSphereOnOrigin> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiSphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group0(),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiVersorEvenOnOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0() + other.group0().xyz()).with_w(other[scalar]),
            // e23, e31, e12, e45
            (other.group1().xyz() + self.group1().xyz()).with_w(self[e45]),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(other[e1234] + self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<Circle> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleAligningOrigin> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: CircleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleAtInfinity> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: CircleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleAtOrigin> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: CircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleOnOrigin> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: CircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleOrthogonalOrigin> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: CircleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotor> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotorAligningOrigin> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotorAligningOriginAtInfinity> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotorAtInfinity> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: CircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotorOnOrigin> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: CircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Dipole> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn add(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0() + self.group0(),
            // e23, e31, e12, e45
            other.group1() + self.group1(),
            // e15, e25, e35, e1234
            (other.group2() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<Dipole> for DipoleInversion {
    fn add_assign(&mut self, other: Dipole) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0() + self.group0(),
            // e23, e31, e12, e45
            other.group1() + self.group1(),
            // e15, e25, e35, e1234
            (other.group2() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<DipoleAligningOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        7        0        0
    fn add(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() + other.group0().xyz(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(other[e45] + self[e45]),
            // e15, e25, e35, e1234
            (other.group1() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<DipoleAligningOrigin> for DipoleInversion {
    fn add_assign(&mut self, other: DipoleAligningOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() + other.group0().xyz(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(other[e45] + self[e45]),
            // e15, e25, e35, e1234
            (other.group1() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<DipoleAtInfinity> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            other.group0() + self.group1(),
            // e15, e25, e35, e1234
            (other.group1() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<DipoleAtInfinity> for DipoleInversion {
    fn add_assign(&mut self, other: DipoleAtInfinity) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            other.group0() + self.group1(),
            // e15, e25, e35, e1234
            (other.group1() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<DipoleAtOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0() + self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            (other.group1() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<DipoleAtOrigin> for DipoleInversion {
    fn add_assign(&mut self, other: DipoleAtOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0() + self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            (other.group1() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<DipoleInversion> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       15        0        0
    fn add(self, other: DipoleInversion) -> Self::Output {
        return DipoleInversion::from_groups(
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
impl std::ops::Add<DipoleInversionAligningOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       15        0        0
    fn add(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() + other.group0().xyz(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35, e1234
            self.group2() + other.group1(),
            // e4235, e4315, e4125, e3215
            self.group3() + other.group2(),
        );
    }
}
impl std::ops::AddAssign<DipoleInversionAligningOrigin> for DipoleInversion {
    fn add_assign(&mut self, other: DipoleInversionAligningOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() + other.group0().xyz(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35, e1234
            self.group2() + other.group1(),
            // e4235, e4315, e4125, e3215
            self.group3() + other.group2(),
        );
    }
}
impl std::ops::Add<DipoleInversionAtInfinity> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       11        0        0
    fn add(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() + other.group0(),
            // e15, e25, e35, e1234
            (other.group1() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3() + other.group2(),
        );
    }
}
impl std::ops::AddAssign<DipoleInversionAtInfinity> for DipoleInversion {
    fn add_assign(&mut self, other: DipoleInversionAtInfinity) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() + other.group0(),
            // e15, e25, e35, e1234
            (other.group1() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3() + other.group2(),
        );
    }
}
impl std::ops::Add<DipoleInversionAtOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       11        0        0
    fn add(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() + other.group0().xyz(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() + other.group1(),
            // e4235, e4315, e4125, e3215
            self.group3() + Simd32x3::from(0.0).with_w(other[e3215]),
        );
    }
}
impl std::ops::AddAssign<DipoleInversionAtOrigin> for DipoleInversion {
    fn add_assign(&mut self, other: DipoleInversionAtOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() + other.group0().xyz(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() + other.group1(),
            // e4235, e4315, e4125, e3215
            self.group3() + Simd32x3::from(0.0).with_w(other[e3215]),
        );
    }
}
impl std::ops::Add<DipoleInversionOnOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       15        0        0
    fn add(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() + other.group0().xyz(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], 0.0]) + self.group3(),
        );
    }
}
impl std::ops::AddAssign<DipoleInversionOnOrigin> for DipoleInversion {
    fn add_assign(&mut self, other: DipoleInversionOnOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() + other.group0().xyz(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], 0.0]) + self.group3(),
        );
    }
}
impl std::ops::Add<DipoleInversionOrthogonalOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       14        0        0
    fn add(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() + other.group0().xyz(),
            // e23, e31, e12, e45
            (other.group1() + self.group1().xyz()).with_w(self[e45]),
            // e15, e25, e35, e1234
            self.group2() + other.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() + Simd32x3::from(0.0).with_w(other[e3215]),
        );
    }
}
impl std::ops::AddAssign<DipoleInversionOrthogonalOrigin> for DipoleInversion {
    fn add_assign(&mut self, other: DipoleInversionOrthogonalOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() + other.group0().xyz(),
            // e23, e31, e12, e45
            (other.group1() + self.group1().xyz()).with_w(self[e45]),
            // e15, e25, e35, e1234
            self.group2() + other.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() + Simd32x3::from(0.0).with_w(other[e3215]),
        );
    }
}
impl std::ops::Add<DipoleOnOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() + other.group0().xyz(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<DipoleOnOrigin> for DipoleInversion {
    fn add_assign(&mut self, other: DipoleOnOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() + other.group0().xyz(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<DipoleOrthogonalOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        3        0        0
    // no simd        9        0        0
    fn add(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() + other.group0(),
            // e23, e31, e12, e45
            (other.group1() + self.group1().xyz()).with_w(self[e45]),
            // e15, e25, e35, e1234
            (other.group2() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<DipoleOrthogonalOrigin> for DipoleInversion {
    fn add_assign(&mut self, other: DipoleOrthogonalOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() + other.group0(),
            // e23, e31, e12, e45
            (other.group1() + self.group1().xyz()).with_w(self[e45]),
            // e15, e25, e35, e1234
            (other.group2() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<DualNum> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<FlatOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<FlatOrigin> for DipoleInversion {
    fn add_assign(&mut self, other: FlatOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<FlatPoint> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35, e1234
            self.group2() + other.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<FlatPoint> for DipoleInversion {
    fn add_assign(&mut self, other: FlatPoint) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35, e1234
            self.group2() + other.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<FlatPointAtInfinity> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: FlatPointAtInfinity) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            (other.group0() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<FlatPointAtInfinity> for DipoleInversion {
    fn add_assign(&mut self, other: FlatPointAtInfinity) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            (other.group0() + self.group2().xyz()).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<Flector> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        3        0        0
    // no simd       12        0        0
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35, e1234
            self.group2() + other.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3() + other.group1(),
        );
    }
}
impl std::ops::AddAssign<Flector> for DipoleInversion {
    fn add_assign(&mut self, other: Flector) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35, e1234
            self.group2() + other.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3() + other.group1(),
        );
    }
}
impl std::ops::Add<FlectorAtInfinity> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() + other.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3() + Simd32x3::from(0.0).with_w(other[e3215]),
        );
    }
}
impl std::ops::AddAssign<FlectorAtInfinity> for DipoleInversion {
    fn add_assign(&mut self, other: FlectorAtInfinity) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() + other.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3() + Simd32x3::from(0.0).with_w(other[e3215]),
        );
    }
}
impl std::ops::Add<FlectorOnOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], 0.0]) + self.group3(),
        );
    }
}
impl std::ops::AddAssign<FlectorOnOrigin> for DipoleInversion {
    fn add_assign(&mut self, other: FlectorOnOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], 0.0]) + self.group3(),
        );
    }
}
impl std::ops::Add<Horizon> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() + Simd32x3::from(0.0).with_w(other[e3215]),
        );
    }
}
impl std::ops::AddAssign<Horizon> for DipoleInversion {
    fn add_assign(&mut self, other: Horizon) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() + Simd32x3::from(0.0).with_w(other[e3215]),
        );
    }
}
impl std::ops::Add<Infinity> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5],
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Line> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<LineAtInfinity> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: LineAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<LineOnOrigin> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: LineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Motor> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5],
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MotorAtInfinity> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: MotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5],
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MotorOnOrigin> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: MotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MultiVector> for DipoleInversion {
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
            other.group1(),
            // e5
            other[e5],
            // e41, e42, e43, e45
            other.group3() + self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], other[e4315], other[e4125]]) + other.group9().xy().with_zw(self[e4315], self[e4125]),
            // e3215
            self[e3215] + other[e3215],
        );
    }
}
impl std::ops::Add<MysteryCircle> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: MysteryCircle) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MysteryCircleRotor> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MysteryDipole> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: MysteryDipole) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() + other.group0(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<MysteryDipole> for DipoleInversion {
    fn add_assign(&mut self, other: MysteryDipole) {
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() + other.group0(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<MysteryDipoleInversion> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() + other.group0(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            (other.group1() + self.group3().xyz()).with_w(self[e3215]),
        );
    }
}
impl std::ops::AddAssign<MysteryDipoleInversion> for DipoleInversion {
    fn add_assign(&mut self, other: MysteryDipoleInversion) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() + other.group0(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            (other.group1() + self.group3().xyz()).with_w(self[e3215]),
        );
    }
}
impl std::ops::Add<MysteryVersorEven> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: MysteryVersorEven) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MysteryVersorOdd> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(other[scalar]),
            // e23, e31, e12, e45
            self.group1() + other.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], 0.0]) + self.group3(),
        );
    }
}
impl std::ops::Add<NullCircleAtOrigin> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: NullCircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<NullDipoleAtOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: NullDipoleAtOrigin) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() + other.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<NullDipoleAtOrigin> for DipoleInversion {
    fn add_assign(&mut self, other: NullDipoleAtOrigin) {
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() + other.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<NullDipoleInversionAtOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() + other.group0().xyz(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<NullDipoleInversionAtOrigin> for DipoleInversion {
    fn add_assign(&mut self, other: NullDipoleInversionAtOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() + other.group0().xyz(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<NullSphereAtOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<NullSphereAtOrigin> for DipoleInversion {
    fn add_assign(&mut self, other: NullSphereAtOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<NullVersorEvenAtOrigin> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Origin> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Plane> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: Plane) -> Self::Output {
        return DipoleInversion::from_groups(
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
impl std::ops::Add<PlaneOnOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            (other.group0() + self.group3().xyz()).with_w(self[e3215]),
        );
    }
}
impl std::ops::AddAssign<PlaneOnOrigin> for DipoleInversion {
    fn add_assign(&mut self, other: PlaneOnOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            (other.group0() + self.group3().xyz()).with_w(self[e3215]),
        );
    }
}
impl std::ops::Add<RoundPoint> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group0(),
            // e5
            other[e5],
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<RoundPointAtOrigin> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Scalar> for DipoleInversion {
    type Output = VersorOdd;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(other[scalar]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<Sphere> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3() + other.group0(),
        );
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
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3() + other.group0(),
        );
    }
}
impl std::ops::Add<SphereAtOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(self[e1234] + other[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3().xyz().with_w(self[e3215] + other[e3215]),
        );
    }
}
impl std::ops::AddAssign<SphereAtOrigin> for DipoleInversion {
    fn add_assign(&mut self, other: SphereAtOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(self[e1234] + other[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3().xyz().with_w(self[e3215] + other[e3215]),
        );
    }
}
impl std::ops::Add<SphereOnOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3() + other.group0().xyz().with_w(0.0),
        );
    }
}
impl std::ops::AddAssign<SphereOnOrigin> for DipoleInversion {
    fn add_assign(&mut self, other: SphereOnOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3() + other.group0().xyz().with_w(0.0),
        );
    }
}
impl std::ops::Add<VersorEven> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            other.group3(),
            // e5
            other[e5],
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenAligningOrigin> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenAtInfinity> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenAtOrigin> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenOnOrigin> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: VersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenOrthogonalOrigin> for DipoleInversion {
    type Output = MultiVector;
    fn add(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group2(),
            // e5
            other[e5],
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorOdd> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       15        0        0
    fn add(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0() + other.group0().xyz()).with_w(other[scalar]),
            // e23, e31, e12, e45
            self.group1() + other.group1(),
            // e15, e25, e35, e1234
            self.group2() + other.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() + other.group3(),
        );
    }
}
impl std::ops::Add<VersorOddAtInfinity> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        3        0        0
    // no simd       12        0        0
    fn add(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(other[scalar]),
            // e23, e31, e12, e45
            self.group1() + other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15], other[e25], other[e35], 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() + other.group2(),
        );
    }
}
impl std::ops::Add<VersorOddOrthogonalOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       15        0        0
    fn add(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0() + other.group0().xyz()).with_w(other[scalar]),
            // e23, e31, e12, e45
            self.group1() + other.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2() + other.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() + Simd32x3::from(0.0).with_w(other[e3215]),
        );
    }
}

impl From<AntiCircleOnOrigin> for DipoleInversion {
    fn from(from_anti_circle_on_origin: AntiCircleOnOrigin) -> Self {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            from_anti_circle_on_origin.group0(),
            // e23, e31, e12, e45
            from_anti_circle_on_origin.group1().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiLine> for DipoleInversion {
    fn from(from_anti_line: AntiLine) -> Self {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            from_anti_line.group0().with_w(0.0),
            // e15, e25, e35, e1234
            from_anti_line.group1().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiLineOnOrigin> for DipoleInversion {
    fn from(from_anti_line_on_origin: AntiLineOnOrigin) -> Self {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            from_anti_line_on_origin.group0().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<Dipole> for DipoleInversion {
    fn from(from_dipole: Dipole) -> Self {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            from_dipole.group0(),
            // e23, e31, e12, e45
            from_dipole.group1(),
            // e15, e25, e35, e1234
            from_dipole.group2().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<DipoleAligningOrigin> for DipoleInversion {
    fn from(from_dipole_aligning_origin: DipoleAligningOrigin) -> Self {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            from_dipole_aligning_origin.group0().xyz(),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_dipole_aligning_origin[e45]),
            // e15, e25, e35, e1234
            from_dipole_aligning_origin.group1().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<DipoleAtInfinity> for DipoleInversion {
    fn from(from_dipole_at_infinity: DipoleAtInfinity) -> Self {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            from_dipole_at_infinity.group0(),
            // e15, e25, e35, e1234
            from_dipole_at_infinity.group1().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<DipoleAtOrigin> for DipoleInversion {
    fn from(from_dipole_at_origin: DipoleAtOrigin) -> Self {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            from_dipole_at_origin.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            from_dipole_at_origin.group1().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<DipoleInversionAligningOrigin> for DipoleInversion {
    fn from(from_dipole_inversion_aligning_origin: DipoleInversionAligningOrigin) -> Self {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            from_dipole_inversion_aligning_origin.group0().xyz(),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_dipole_inversion_aligning_origin[e45]),
            // e15, e25, e35, e1234
            from_dipole_inversion_aligning_origin.group1(),
            // e4235, e4315, e4125, e3215
            from_dipole_inversion_aligning_origin.group2(),
        );
    }
}

impl From<DipoleInversionAtInfinity> for DipoleInversion {
    fn from(from_dipole_inversion_at_infinity: DipoleInversionAtInfinity) -> Self {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            from_dipole_inversion_at_infinity.group0(),
            // e15, e25, e35, e1234
            from_dipole_inversion_at_infinity.group1().with_w(0.0),
            // e4235, e4315, e4125, e3215
            from_dipole_inversion_at_infinity.group2(),
        );
    }
}

impl From<DipoleInversionAtOrigin> for DipoleInversion {
    fn from(from_dipole_inversion_at_origin: DipoleInversionAtOrigin) -> Self {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            from_dipole_inversion_at_origin.group0().xyz(),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            from_dipole_inversion_at_origin.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(from_dipole_inversion_at_origin[e3215]),
        );
    }
}

impl From<DipoleInversionOnOrigin> for DipoleInversion {
    fn from(from_dipole_inversion_on_origin: DipoleInversionOnOrigin) -> Self {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            from_dipole_inversion_on_origin.group0().xyz(),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_dipole_inversion_on_origin[e45]),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_dipole_inversion_on_origin[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                from_dipole_inversion_on_origin[e4235],
                from_dipole_inversion_on_origin[e4315],
                from_dipole_inversion_on_origin[e4125],
                0.0,
            ]),
        );
    }
}

impl From<DipoleInversionOrthogonalOrigin> for DipoleInversion {
    fn from(from_dipole_inversion_orthogonal_origin: DipoleInversionOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            from_dipole_inversion_orthogonal_origin.group0().xyz(),
            // e23, e31, e12, e45
            from_dipole_inversion_orthogonal_origin.group1().with_w(0.0),
            // e15, e25, e35, e1234
            from_dipole_inversion_orthogonal_origin.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(from_dipole_inversion_orthogonal_origin[e3215]),
        );
    }
}

impl From<DipoleOnOrigin> for DipoleInversion {
    fn from(from_dipole_on_origin: DipoleOnOrigin) -> Self {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            from_dipole_on_origin.group0().xyz(),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_dipole_on_origin[e45]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<DipoleOrthogonalOrigin> for DipoleInversion {
    fn from(from_dipole_orthogonal_origin: DipoleOrthogonalOrigin) -> Self {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            from_dipole_orthogonal_origin.group0(),
            // e23, e31, e12, e45
            from_dipole_orthogonal_origin.group1().with_w(0.0),
            // e15, e25, e35, e1234
            from_dipole_orthogonal_origin.group2().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<FlatOrigin> for DipoleInversion {
    fn from(from_flat_origin: FlatOrigin) -> Self {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_flat_origin[e45]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<FlatPoint> for DipoleInversion {
    fn from(from_flat_point: FlatPoint) -> Self {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_flat_point[e45]),
            // e15, e25, e35, e1234
            from_flat_point.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<FlatPointAtInfinity> for DipoleInversion {
    fn from(from_flat_point_at_infinity: FlatPointAtInfinity) -> Self {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            from_flat_point_at_infinity.group0().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<Flector> for DipoleInversion {
    fn from(from_flector: Flector) -> Self {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_flector[e45]),
            // e15, e25, e35, e1234
            from_flector.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            from_flector.group1(),
        );
    }
}

impl From<FlectorAtInfinity> for DipoleInversion {
    fn from(from_flector_at_infinity: FlectorAtInfinity) -> Self {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            from_flector_at_infinity.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(from_flector_at_infinity[e3215]),
        );
    }
}

impl From<FlectorOnOrigin> for DipoleInversion {
    fn from(from_flector_on_origin: FlectorOnOrigin) -> Self {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_flector_on_origin[e45]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([from_flector_on_origin[e4235], from_flector_on_origin[e4315], from_flector_on_origin[e4125], 0.0]),
        );
    }
}

impl From<Horizon> for DipoleInversion {
    fn from(from_horizon: Horizon) -> Self {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(from_horizon[e3215]),
        );
    }
}

impl From<MysteryDipole> for DipoleInversion {
    fn from(from_mystery_dipole: MysteryDipole) -> Self {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            from_mystery_dipole.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<MysteryDipoleInversion> for DipoleInversion {
    fn from(from_mystery_dipole_inversion: MysteryDipoleInversion) -> Self {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            from_mystery_dipole_inversion.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            from_mystery_dipole_inversion.group1().with_w(0.0),
        );
    }
}

impl From<NullDipoleAtOrigin> for DipoleInversion {
    fn from(from_null_dipole_at_origin: NullDipoleAtOrigin) -> Self {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            from_null_dipole_at_origin.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<NullDipoleInversionAtOrigin> for DipoleInversion {
    fn from(from_null_dipole_inversion_at_origin: NullDipoleInversionAtOrigin) -> Self {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            from_null_dipole_inversion_at_origin.group0().xyz(),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_null_dipole_inversion_at_origin[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<NullSphereAtOrigin> for DipoleInversion {
    fn from(from_null_sphere_at_origin: NullSphereAtOrigin) -> Self {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_null_sphere_at_origin[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<Plane> for DipoleInversion {
    fn from(from_plane: Plane) -> Self {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            from_plane.group0(),
        );
    }
}

impl From<PlaneOnOrigin> for DipoleInversion {
    fn from(from_plane_on_origin: PlaneOnOrigin) -> Self {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            from_plane_on_origin.group0().with_w(0.0),
        );
    }
}

impl From<Sphere> for DipoleInversion {
    fn from(from_sphere: Sphere) -> Self {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_sphere[e1234]),
            // e4235, e4315, e4125, e3215
            from_sphere.group0(),
        );
    }
}

impl From<SphereAtOrigin> for DipoleInversion {
    fn from(from_sphere_at_origin: SphereAtOrigin) -> Self {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_sphere_at_origin[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(from_sphere_at_origin[e3215]),
        );
    }
}

impl From<SphereOnOrigin> for DipoleInversion {
    fn from(from_sphere_on_origin: SphereOnOrigin) -> Self {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_sphere_on_origin[e1234]),
            // e4235, e4315, e4125, e3215
            from_sphere_on_origin.group0().xyz().with_w(0.0),
        );
    }
}
impl std::ops::Mul<AntiCircleOnOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       51        0
    //    simd3        0        4        0
    //    simd4       11        7        0
    // Totals...
    // yes simd       44       62        0
    //  no simd       77       91        0
    fn mul(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       45       70        0
    //    simd3        0        9        0
    //    simd4       26       17        0
    // Totals...
    // yes simd       71       96        0
    //  no simd      149      165        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       42       68        0
    //    simd3        0       10        0
    //    simd4       23       13        0
    // Totals...
    // yes simd       65       91        0
    //  no simd      134      150        0
    fn mul(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOriginAtInfinity> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       56        0
    //    simd3        0       10        0
    //    simd4       15        5        0
    // Totals...
    // yes simd       48       71        0
    //  no simd       93      106        0
    fn mul(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAtInfinity> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       37       63        0
    //    simd3        0       10        0
    //    simd4       17        7        0
    // Totals...
    // yes simd       54       80        0
    //  no simd      105      121        0
    fn mul(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorOnOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       55        0
    //    simd3        0        5        0
    //    simd4       14        9        0
    // Totals...
    // yes simd       50       69        0
    //  no simd       92      106        0
    fn mul(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       62        0
    //    simd3        0       15        0
    //    simd4       45       30        0
    // Totals...
    // yes simd       74      107        0
    //  no simd      209      227        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionAtInfinity> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       37       62        0
    //    simd3        0       12        0
    //    simd4       29       17        0
    // Totals...
    // yes simd       66       91        0
    //  no simd      153      166        0
    fn mul(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOnOrigin> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       32        0
    //    simd3        0        8        0
    //    simd4       24       16        0
    // Totals...
    // yes simd       37       56        0
    //  no simd      109      120        0
    fn mul(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOrthogonalOrigin> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       56        0
    //    simd3        0        9        0
    //    simd4       30       21        0
    // Totals...
    // yes simd       62       86        0
    //  no simd      152      167        0
    fn mul(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleOnOrigin> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       17        0
    //    simd3        0        1        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       18       28        0
    //  no simd       48       60        0
    fn mul(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        4        0
    //    simd3        3        5        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        5       12        0
    //  no simd       14       31        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatOrigin> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       26        0
    fn mul(self, other: AntiFlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       28        0
    //    simd3        3        7        0
    //    simd4        5        4        0
    // Totals...
    // yes simd       23       39        0
    //  no simd       44       65        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       54        0
    //    simd3        0        6        0
    //    simd4       18       12        0
    // Totals...
    // yes simd       54       72        0
    //  no simd      108      120        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlectorOnOrigin> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       24        0
    //    simd3        0        4        0
    //    simd4       10        6        0
    // Totals...
    // yes simd       18       34        0
    //  no simd       48       60        0
    fn mul(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       51        0
    //    simd3        0        8        0
    //    simd4       12        4        0
    // Totals...
    // yes simd       42       63        0
    //  no simd       78       91        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLineOnOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       33        0
    //    simd3        0        4        0
    //    simd4        4        0        0
    // Totals...
    // yes simd       17       37        0
    //  no simd       29       45        0
    fn mul(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       58        0
    //    simd3        0       10        0
    //    simd4       18        8        0
    // Totals...
    // yes simd       51       76        0
    //  no simd      105      120        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotorOnOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       34        0
    //    simd3        0        2        0
    //    simd4        7        5        0
    // Totals...
    // yes simd       23       41        0
    //  no simd       44       60        0
    fn mul(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryCircleRotor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       41        0
    //    simd3        0        5        0
    //    simd4       10        5        0
    // Totals...
    // yes simd       29       51        0
    //  no simd       59       76        0
    fn mul(self, other: AntiMysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryDipoleInversion> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       34        0
    //    simd3        0        9        0
    //    simd4       20       12        0
    // Totals...
    // yes simd       32       55        0
    //  no simd       92      109        0
    fn mul(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       35        0
    //    simd3        0        3        0
    //    simd4        8        5        0
    // Totals...
    // yes simd       20       43        0
    //  no simd       44       64        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlaneOnOrigin> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       31        0
    //    simd3        0        2        0
    //    simd4        5        3        0
    // Totals...
    // yes simd       14       36        0
    //  no simd       29       49        0
    fn mul(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       23        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiSphereOnOrigin> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       37        0
    //    simd3        0        2        0
    //    simd4        7        5        0
    // Totals...
    // yes simd       23       44        0
    //  no simd       44       63        0
    fn mul(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorEvenOnOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       39       57        0
    //    simd3        0        3        0
    //    simd4       17       14        0
    // Totals...
    // yes simd       56       74        0
    //  no simd      107      122        0
    fn mul(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       62        0
    //    simd3        0        9        0
    //    simd4       25       16        0
    // Totals...
    // yes simd       59       87        0
    //  no simd      134      153        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAligningOrigin> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       53        0
    //    simd3        0        9        0
    //    simd4       23       14        0
    // Totals...
    // yes simd       53       76        0
    //  no simd      122      136        0
    fn mul(self, other: CircleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtInfinity> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       54        0
    //    simd3        0        8        0
    //    simd4       14        7        0
    // Totals...
    // yes simd       47       69        0
    //  no simd       89      106        0
    fn mul(self, other: CircleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtOrigin> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       31        0
    //    simd3        0        4        0
    //    simd4       15       12        0
    // Totals...
    // yes simd       29       47        0
    //  no simd       74       91        0
    fn mul(self, other: CircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOnOrigin> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       37        0
    //    simd3        0        3        0
    //    simd4       14       11        0
    // Totals...
    // yes simd       35       51        0
    //  no simd       77       90        0
    fn mul(self, other: CircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOrthogonalOrigin> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       36        0
    //    simd3        0        6        0
    //    simd4       19       13        0
    // Totals...
    // yes simd       33       55        0
    //  no simd       90      106        0
    fn mul(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       61        0
    //    simd3        0       15        0
    //    simd4       29       15        0
    // Totals...
    // yes simd       62       91        0
    //  no simd      149      166        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOrigin> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       56        0
    //    simd3        0       13        0
    //    simd4       26       14        0
    // Totals...
    // yes simd       59       83        0
    //  no simd      137      151        0
    fn mul(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOriginAtInfinity> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       58        0
    //    simd3        0        9        0
    //    simd4       14        5        0
    // Totals...
    // yes simd       48       72        0
    //  no simd       90      105        0
    fn mul(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAtInfinity> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       60        0
    //    simd3        0       11        0
    //    simd4       18        7        0
    // Totals...
    // yes simd       51       78        0
    //  no simd      105      121        0
    fn mul(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorOnOrigin> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       38        0
    //    simd3        0        2        0
    //    simd4       18       16        0
    // Totals...
    // yes simd       38       56        0
    //  no simd       92      108        0
    fn mul(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       42       66        0
    //    simd3        0        8        0
    //    simd4       23       15        0
    // Totals...
    // yes simd       65       89        0
    //  no simd      134      150        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAligningOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       39        0
    //    simd3        0        5        0
    //    simd4       18       13        0
    // Totals...
    // yes simd       38       57        0
    //  no simd       92      106        0
    fn mul(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtInfinity> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       59        0
    //    simd3        0        9        0
    //    simd4       14        5        0
    // Totals...
    // yes simd       48       73        0
    //  no simd       90      106        0
    fn mul(self, other: DipoleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       32        0
    //    simd3        0        4        0
    //    simd4       16       12        0
    // Totals...
    // yes simd       29       48        0
    //  no simd       77       92        0
    fn mul(self, other: DipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       61       89        0
    //    simd3        0       12        0
    //    simd4       37       25        0
    // Totals...
    // yes simd       98      126        0
    //  no simd      209      225        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAligningOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       61        0
    //    simd3        0       13        0
    //    simd4       33       20        0
    // Totals...
    // yes simd       65       94        0
    //  no simd      164      180        0
    fn mul(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtInfinity> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       46       73        0
    //    simd3        0       12        0
    //    simd4       26       14        0
    // Totals...
    // yes simd       72       99        0
    //  no simd      150      165        0
    fn mul(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       39        0
    //    simd3        0        7        0
    //    simd4       22       15        0
    // Totals...
    // yes simd       38       61        0
    //  no simd      104      120        0
    fn mul(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOnOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       52        0
    //    simd3        0        3        0
    //    simd4       18       15        0
    // Totals...
    // yes simd       54       70        0
    //  no simd      108      121        0
    fn mul(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOrthogonalOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       45       76        0
    //    simd3        0       15        0
    //    simd4       26       11        0
    // Totals...
    // yes simd       71      102        0
    //  no simd      149      165        0
    fn mul(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOnOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       26        0
    //    simd3        3        6        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       24       37        0
    //  no simd       45       64        0
    fn mul(self, other: DipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOrthogonalOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       39       66        0
    //    simd3        0       11        0
    //    simd4       20        9        0
    // Totals...
    // yes simd       59       86        0
    //  no simd      119      135        0
    fn mul(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       16        0
    //    simd3        2        3        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        7       21        0
    //  no simd       14       33        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       23        0
    fn mul(self, other: FlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       22        0
    //    simd3        5        7        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       20       34        0
    //  no simd       45       63        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPointAtInfinity> for DipoleInversion {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       29        0
    //    simd3        0        4        0
    //    simd4        6        2        0
    // Totals...
    // yes simd       15       35        0
    //  no simd       33       49        0
    fn mul(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       49        0
    //    simd3        0        5        0
    //    simd4       19       14        0
    // Totals...
    // yes simd       51       68        0
    //  no simd      108      120        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorAtInfinity> for DipoleInversion {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       17        0
    //    simd4       10       11        0
    // Totals...
    // yes simd       21       28        0
    //  no simd       51       61        0
    fn mul(self, other: FlectorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorOnOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       20        0
    //    simd4        9       10        0
    // Totals...
    // yes simd       20       30        0
    //  no simd       47       60        0
    fn mul(self, other: FlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for DipoleInversion {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3       11        0
    //  no simd        3       23        0
    fn mul(self, other: Horizon) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Infinity> for DipoleInversion {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        5        0
    //  no simd        3       15        0
    fn mul(self, other: Infinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       52        0
    //    simd3        0        9        0
    //    simd4       12        3        0
    // Totals...
    // yes simd       42       64        0
    //  no simd       78       91        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineAtInfinity> for DipoleInversion {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       31        0
    //    simd3        0        2        0
    //    simd4        4        2        0
    // Totals...
    // yes simd       21       35        0
    //  no simd       33       45        0
    fn mul(self, other: LineAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineOnOrigin> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       31        0
    //    simd3        0        2        0
    //    simd4        5        3        0
    // Totals...
    // yes simd       14       36        0
    //  no simd       29       49        0
    fn mul(self, other: LineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       37       60        0
    //    simd3        0        8        0
    //    simd4       17        9        0
    // Totals...
    // yes simd       54       77        0
    //  no simd      105      120        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorAtInfinity> for DipoleInversion {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       27       39        0
    //  no simd       48       60        0
    fn mul(self, other: MotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorOnOrigin> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       20        0
    //    simd4        9       10        0
    // Totals...
    // yes simd       20       30        0
    //  no simd       47       60        0
    fn mul(self, other: MotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       66       99        0
    //    simd2       11       12        0
    //    simd3       56       75        0
    //    simd4       48       33        0
    // Totals...
    // yes simd      181      219        0
    //  no simd      448      480        0
    fn mul(self, other: MultiVector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircle> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       32        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       23       39        0
    //  no simd       44       60        0
    fn mul(self, other: MysteryCircle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircleRotor> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       32        0
    //    simd3        0        1        0
    //    simd4       12       11        0
    // Totals...
    // yes simd       23       44        0
    //  no simd       59       79        0
    fn mul(self, other: MysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipole> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       33        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       23       40        0
    //  no simd       44       61        0
    fn mul(self, other: MysteryDipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipoleInversion> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       55        0
    //    simd3        0       10        0
    //    simd4       15        5        0
    // Totals...
    // yes simd       44       70        0
    //  no simd       89      105        0
    fn mul(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorEven> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       23        0
    //    simd3        0        3        0
    //    simd4       24       22        0
    // Totals...
    // yes simd       36       48        0
    //  no simd      108      120        0
    fn mul(self, other: MysteryVersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorOdd> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       53        0
    //    simd3        0        5        0
    //    simd4       18       13        0
    // Totals...
    // yes simd       50       71        0
    //  no simd      104      120        0
    fn mul(self, other: MysteryVersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullCircleAtOrigin> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       17        0
    //    simd3        0        1        0
    //    simd4        8        7        0
    // Totals...
    // yes simd       13       25        0
    //  no simd       37       48        0
    fn mul(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleAtOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       20        0
    //    simd3        0        3        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       18       27        0
    //  no simd       36       45        0
    fn mul(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleInversionAtOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd3        0        4        0
    //    simd4        9        6        0
    // Totals...
    // yes simd       25       34        0
    //  no simd       52       60        0
    fn mul(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullSphereAtOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        1        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        7        0
    //  no simd        3       19        0
    fn mul(self, other: NullSphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullVersorEvenAtOrigin> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4       11       12        0
    // Totals...
    // yes simd       19       24        0
    //  no simd       52       60        0
    fn mul(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Origin> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1        7        0
    //  no simd        3       23        0
    fn mul(self, other: Origin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       37        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       26       43        0
    //  no simd       44       61        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<PlaneOnOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       32        0
    //    simd3        0        3        0
    //    simd4        5        2        0
    // Totals...
    // yes simd       14       37        0
    //  no simd       29       49        0
    fn mul(self, other: PlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       40        0
    //    simd3        0        2        0
    //    simd4       10        8        0
    // Totals...
    // yes simd       29       50        0
    //  no simd       59       78        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPointAtOrigin> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       15        0
    //    simd3        2        4        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        4       21        0
    //  no simd       14       35        0
    fn mul(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn mul(self, other: Scalar) -> Self::Output {
        return self.geometric_product(other);
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
    //           add/sub      mul      div
    //      f32       19       37        0
    //    simd3        0        1        0
    //    simd4       10        9        0
    // Totals...
    // yes simd       29       47        0
    //  no simd       59       76        0
    fn mul(self, other: Sphere) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereAtOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       15        0
    //    simd3        2        4        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        4       21        0
    //  no simd       14       35        0
    fn mul(self, other: SphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereOnOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       28        0
    //    simd4        9        9        0
    // Totals...
    // yes simd       17       37        0
    //  no simd       44       64        0
    fn mul(self, other: SphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       66        0
    //    simd3        0       18        0
    //    simd4       47       30        0
    // Totals...
    // yes simd       83      114        0
    //  no simd      224      240        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAligningOrigin> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       63        0
    //    simd3        0       15        0
    //    simd4       33       18        0
    // Totals...
    // yes simd       66       96        0
    //  no simd      165      180        0
    fn mul(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtInfinity> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       66        0
    //    simd3        0       14        0
    //    simd4       32       18        0
    // Totals...
    // yes simd       72       98        0
    //  no simd      168      180        0
    fn mul(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtOrigin> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       42        0
    //    simd3        0       10        0
    //    simd4       22       12        0
    // Totals...
    // yes simd       39       64        0
    //  no simd      105      120        0
    fn mul(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOnOrigin> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       30        0
    //    simd3        0        1        0
    //    simd4       23       22        0
    // Totals...
    // yes simd       41       53        0
    //  no simd      110      121        0
    fn mul(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOrthogonalOrigin> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       48        0
    //    simd3        0       12        0
    //    simd4       35       24        0
    // Totals...
    // yes simd       62       84        0
    //  no simd      167      180        0
    fn mul(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       64       92        0
    //    simd3        0       12        0
    //    simd4       40       28        0
    // Totals...
    // yes simd      104      132        0
    //  no simd      224      240        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddAtInfinity> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       49       71        0
    //    simd3        0        7        0
    //    simd4       29       22        0
    // Totals...
    // yes simd       78      100        0
    //  no simd      165      180        0
    fn mul(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddOrthogonalOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       75        0
    //    simd3        0       11        0
    //    simd4       29       18        0
    // Totals...
    // yes simd       77      104        0
    //  no simd      164      180        0
    fn mul(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Neg for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn neg(self) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group3() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Not for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiCircleOnOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        7        3        0
    fn sub(self, other: AntiCircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() - other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<AntiCircleOnOrigin> for DipoleInversion {
    fn sub_assign(&mut self, other: AntiCircleOnOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() - other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<AntiCircleRotor> for DipoleInversion {
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
            (self.group0() - other.group0()).with_w(other[scalar]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            self.group1() - other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        1        0        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        3        7        0
    //  no simd       11       10        0
    fn sub(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0() - other.group0()).with_w(other[scalar]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOriginAtInfinity> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        7        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        8        7        0
    fn sub(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(other[scalar] * -1.0),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAtInfinity> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        4        0
    fn sub(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(other[scalar] * -1.0),
            // e23, e31, e12, e45
            self.group1() - other.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorOnOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        1        0        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        7        7        0
    fn sub(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0() - other.group0().xyz()).with_w(other[scalar]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversion> for DipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(other[e4]) * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionAtInfinity> for DipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (other.group2().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOnOrigin> for DipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group1().yzwx() * Simd32x4::from(-1.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOrthogonalOrigin> for DipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDipoleOnOrigin> for DipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDualNum> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        4        2        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(other[scalar] * -1.0),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<AntiFlatOrigin> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiFlatPoint> for DipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiFlector> for DipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiFlectorOnOrigin> for DipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (other.group0().yzw() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiLine> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        8        6        0
    fn sub(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<AntiLine> for DipoleInversion {
    fn sub_assign(&mut self, other: AntiLine) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<AntiLineOnOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        4        3        0
    fn sub(self, other: AntiLineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<AntiLineOnOrigin> for DipoleInversion {
    fn sub_assign(&mut self, other: AntiLineOnOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<AntiMotor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        8        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        3        8        0
    //  no simd       12        8        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(other[scalar] * -1.0),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() + Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
        );
    }
}
impl std::ops::Sub<AntiMotorOnOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        4        4        0
    fn sub(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(other[scalar] * -1.0),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<AntiMysteryCircleRotor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(other[scalar] * -1.0),
            // e23, e31, e12, e45
            self.group1() - other.group0(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<AntiMysteryDipoleInversion> for DipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (other.group1() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiPlane> for DipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiPlaneOnOrigin> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: AntiPlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiScalar> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiSphereOnOrigin> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiSphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group0() * Simd32x4::from(-1.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiVersorEvenOnOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        1        0        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd       11        8        0
    fn sub(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0() - other.group0().xyz()).with_w(other[scalar]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<Circle> for DipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleAligningOrigin> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn sub(self, other: CircleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleAtInfinity> for DipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleAtOrigin> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: CircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleOnOrigin> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: CircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleOrthogonalOrigin> for DipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotor> for DipoleInversion {
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
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOrigin> for DipoleInversion {
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
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOriginAtInfinity> for DipoleInversion {
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
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotorAtInfinity> for DipoleInversion {
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
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotorOnOrigin> for DipoleInversion {
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
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Dipole> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        3        0
    //  no simd       11        3        0
    fn sub(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() - other.group0(),
            // e23, e31, e12, e45
            self.group1() - other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<Dipole> for DipoleInversion {
    fn sub_assign(&mut self, other: Dipole) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() - other.group0(),
            // e23, e31, e12, e45
            self.group1() - other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<DipoleAligningOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        4        0
    //  no simd       11        4        0
    fn sub(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() - other.group0().xyz(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<DipoleAligningOrigin> for DipoleInversion {
    fn sub_assign(&mut self, other: DipoleAligningOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() - other.group0().xyz(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<DipoleAtInfinity> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        8        3        0
    fn sub(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() - other.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<DipoleAtInfinity> for DipoleInversion {
    fn sub_assign(&mut self, other: DipoleAtInfinity) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() - other.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<DipoleAtOrigin> for DipoleInversion {
    type Output = DipoleInversion;
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
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() - other.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<DipoleAtOrigin> for DipoleInversion {
    fn sub_assign(&mut self, other: DipoleAtOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() - other.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<DipoleInversion> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       15        0        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        return DipoleInversion::from_groups(
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
impl std::ops::Sub<DipoleInversionAligningOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        0        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        4        1        0
    //  no simd       15        1        0
    fn sub(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() - other.group0().xyz(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35, e1234
            self.group2() - other.group1(),
            // e4235, e4315, e4125, e3215
            self.group3() - other.group2(),
        );
    }
}
impl std::ops::SubAssign<DipoleInversionAligningOrigin> for DipoleInversion {
    fn sub_assign(&mut self, other: DipoleInversionAligningOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() - other.group0().xyz(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35, e1234
            self.group2() - other.group1(),
            // e4235, e4315, e4125, e3215
            self.group3() - other.group2(),
        );
    }
}
impl std::ops::Sub<DipoleInversionAtInfinity> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        3        3        0
    //  no simd       12        3        0
    fn sub(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() - other.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() - other.group2(),
        );
    }
}
impl std::ops::SubAssign<DipoleInversionAtInfinity> for DipoleInversion {
    fn sub_assign(&mut self, other: DipoleInversionAtInfinity) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() - other.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() - other.group2(),
        );
    }
}
impl std::ops::Sub<DipoleInversionAtOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        1        0
    //  no simd       11        1        0
    fn sub(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() - other.group0().xyz(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() - other.group1(),
            // e4235, e4315, e4125, e3215
            self.group3() + Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
        );
    }
}
impl std::ops::SubAssign<DipoleInversionAtOrigin> for DipoleInversion {
    fn sub_assign(&mut self, other: DipoleInversionAtOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() - other.group0().xyz(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() - other.group1(),
            // e4235, e4315, e4125, e3215
            self.group3() + Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
        );
    }
}
impl std::ops::Sub<DipoleInversionOnOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        1        0        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        4        5        0
    //  no simd       15        5        0
    fn sub(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() - other.group0().xyz(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0, 0.0]) + self.group3(),
        );
    }
}
impl std::ops::SubAssign<DipoleInversionOnOrigin> for DipoleInversion {
    fn sub_assign(&mut self, other: DipoleInversionOnOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() - other.group0().xyz(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0, 0.0]) + self.group3(),
        );
    }
}
impl std::ops::Sub<DipoleInversionOrthogonalOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        1        0        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        4        4        0
    //  no simd       15        4        0
    fn sub(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() - other.group0().xyz(),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            self.group2() - other.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() + Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
        );
    }
}
impl std::ops::SubAssign<DipoleInversionOrthogonalOrigin> for DipoleInversion {
    fn sub_assign(&mut self, other: DipoleInversionOrthogonalOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() - other.group0().xyz(),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            self.group2() - other.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() + Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
        );
    }
}
impl std::ops::Sub<DipoleOnOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        7        1        0
    fn sub(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() - other.group0().xyz(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<DipoleOnOrigin> for DipoleInversion {
    fn sub_assign(&mut self, other: DipoleOnOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() - other.group0().xyz(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<DipoleOrthogonalOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        6        0
    //  no simd       11        6        0
    fn sub(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() - other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<DipoleOrthogonalOrigin> for DipoleInversion {
    fn sub_assign(&mut self, other: DipoleOrthogonalOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() - other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<DualNum> for DipoleInversion {
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
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<FlatOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<FlatOrigin> for DipoleInversion {
    fn sub_assign(&mut self, other: FlatOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<FlatPoint> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        4        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<FlatPoint> for DipoleInversion {
    fn sub_assign(&mut self, other: FlatPoint) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<FlatPointAtInfinity> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        4        3        0
    fn sub(self, other: FlatPointAtInfinity) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<FlatPointAtInfinity> for DipoleInversion {
    fn sub_assign(&mut self, other: FlatPointAtInfinity) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<Flector> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        3        4        0
    //  no simd       12        4        0
    fn sub(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() - other.group1(),
        );
    }
}
impl std::ops::SubAssign<Flector> for DipoleInversion {
    fn sub_assign(&mut self, other: Flector) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() - other.group1(),
        );
    }
}
impl std::ops::Sub<FlectorAtInfinity> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        4        0
    fn sub(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() + Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
        );
    }
}
impl std::ops::SubAssign<FlectorAtInfinity> for DipoleInversion {
    fn sub_assign(&mut self, other: FlectorAtInfinity) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() + Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
        );
    }
}
impl std::ops::Sub<FlectorOnOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        4        0
    fn sub(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0, 0.0]) + self.group3(),
        );
    }
}
impl std::ops::SubAssign<FlectorOnOrigin> for DipoleInversion {
    fn sub_assign(&mut self, other: FlectorOnOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0, 0.0]) + self.group3(),
        );
    }
}
impl std::ops::Sub<Horizon> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() + Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
        );
    }
}
impl std::ops::SubAssign<Horizon> for DipoleInversion {
    fn sub_assign(&mut self, other: Horizon) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() + Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
        );
    }
}
impl std::ops::Sub<Infinity> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Line> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<LineAtInfinity> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: LineAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<LineOnOrigin> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: LineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Motor> for DipoleInversion {
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
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MotorAtInfinity> for DipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MotorOnOrigin> for DipoleInversion {
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
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MultiVector> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        1        0
    //    simd2        0        1        0
    //    simd3        2        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        5        6        0
    //  no simd       15       17        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            other.group0() * Simd32x2::from(-1.0),
            // e1, e2, e3, e4
            other.group1() * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]) - other.group3(),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]) - other.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<MysteryCircle> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: MysteryCircle) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MysteryCircleRotor> for DipoleInversion {
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
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MysteryDipole> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: MysteryDipole) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() - other.group0(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<MysteryDipole> for DipoleInversion {
    fn sub_assign(&mut self, other: MysteryDipole) {
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() - other.group0(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<MysteryDipoleInversion> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        8        3        0
    fn sub(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() - other.group0(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0, 0.0]) + self.group3(),
        );
    }
}
impl std::ops::SubAssign<MysteryDipoleInversion> for DipoleInversion {
    fn sub_assign(&mut self, other: MysteryDipoleInversion) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() - other.group0(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0, 0.0]) + self.group3(),
        );
    }
}
impl std::ops::Sub<MysteryVersorEven> for DipoleInversion {
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
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            (other.group0().yzw() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MysteryVersorOdd> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        4        0
    fn sub(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(other[scalar] * -1.0),
            // e23, e31, e12, e45
            self.group1() - other.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0, 0.0]) + self.group3(),
        );
    }
}
impl std::ops::Sub<NullCircleAtOrigin> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: NullCircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<NullDipoleAtOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn sub(self, other: NullDipoleAtOrigin) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() - other.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<NullDipoleAtOrigin> for DipoleInversion {
    fn sub_assign(&mut self, other: NullDipoleAtOrigin) {
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() - other.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<NullDipoleInversionAtOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        7        1        0
    fn sub(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() - other.group0().xyz(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<NullDipoleInversionAtOrigin> for DipoleInversion {
    fn sub_assign(&mut self, other: NullDipoleInversionAtOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() - other.group0().xyz(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<NullSphereAtOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<NullSphereAtOrigin> for DipoleInversion {
    fn sub_assign(&mut self, other: NullSphereAtOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<NullVersorEvenAtOrigin> for DipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Origin> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Plane> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: Plane) -> Self::Output {
        return DipoleInversion::from_groups(
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
impl std::ops::Sub<PlaneOnOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        4        3        0
    fn sub(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0, 0.0]) + self.group3(),
        );
    }
}
impl std::ops::SubAssign<PlaneOnOrigin> for DipoleInversion {
    fn sub_assign(&mut self, other: PlaneOnOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0, 0.0]) + self.group3(),
        );
    }
}
impl std::ops::Sub<RoundPoint> for DipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group0() * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<RoundPointAtOrigin> for DipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Scalar> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(other[scalar] * -1.0),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<Sphere> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        8        1        0
    fn sub(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            self.group3() - other.group0(),
        );
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
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            self.group3() - other.group0(),
        );
    }
}
impl std::ops::Sub<SphereAtOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        8        2        0
    fn sub(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            self.group3() + Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
        );
    }
}
impl std::ops::SubAssign<SphereAtOrigin> for DipoleInversion {
    fn sub_assign(&mut self, other: SphereAtOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            self.group3() + Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
        );
    }
}
impl std::ops::Sub<SphereOnOrigin> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        4        0
    fn sub(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0, 0.0]) + self.group3(),
        );
    }
}
impl std::ops::SubAssign<SphereOnOrigin> for DipoleInversion {
    fn sub_assign(&mut self, other: SphereOnOrigin) {
        use crate::elements::*;
        *self = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0, 0.0]) + self.group3(),
        );
    }
}
impl std::ops::Sub<VersorEven> for DipoleInversion {
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
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            other.group3() * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenAligningOrigin> for DipoleInversion {
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
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenAtInfinity> for DipoleInversion {
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
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            (other.group0().yzw() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenAtOrigin> for DipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenOnOrigin> for DipoleInversion {
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
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenOrthogonalOrigin> for DipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group2() * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            self.group0().with_w(self[e45]),
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
            Simd32x4::from([self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorOdd> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        3        1        0
    // Totals...
    // yes simd        4        1        0
    //  no simd       15        4        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0() - other.group0().xyz()).with_w(other[scalar]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            self.group1() - other.group1(),
            // e15, e25, e35, e1234
            self.group2() - other.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() - other.group3(),
        );
    }
}
impl std::ops::Sub<VersorOddAtInfinity> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        3        4        0
    //  no simd       12        4        0
    fn sub(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().with_w(other[scalar] * -1.0),
            // e23, e31, e12, e45
            self.group1() - other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, 0.0]) + self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() - other.group2(),
        );
    }
}
impl std::ops::Sub<VersorOddOrthogonalOrigin> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        1        0        0
    //    simd4        3        1        0
    // Totals...
    // yes simd        4        5        0
    //  no simd       15        8        0
    fn sub(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0() - other.group0().xyz()).with_w(other[scalar]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
            // e15, e25, e35, e1234
            self.group2() - other.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() + Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
        );
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
        return Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            anti_circle_rotor.group0(),
            // e23, e31, e12, e45
            anti_circle_rotor.group1(),
            // e15, e25, e35, e1234
            anti_circle_rotor.group2().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiCircleRotorAligningOrigin> for DipoleInversion {
    type Error = String;
    fn try_from(anti_circle_rotor_aligning_origin: AntiCircleRotorAligningOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor_aligning_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotorAligningOrigin do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            anti_circle_rotor_aligning_origin.group0(),
            // e23, e31, e12, e45
            anti_circle_rotor_aligning_origin.group1().with_w(0.0),
            // e15, e25, e35, e1234
            anti_circle_rotor_aligning_origin.group2().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiCircleRotorAligningOriginAtInfinity> for DipoleInversion {
    type Error = String;
    fn try_from(anti_circle_rotor_aligning_origin_at_infinity: AntiCircleRotorAligningOriginAtInfinity) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor_aligning_origin_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotorAligningOriginAtInfinity do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            anti_circle_rotor_aligning_origin_at_infinity.group0().with_w(0.0),
            // e15, e25, e35, e1234
            anti_circle_rotor_aligning_origin_at_infinity.group1().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiCircleRotorAtInfinity> for DipoleInversion {
    type Error = String;
    fn try_from(anti_circle_rotor_at_infinity: AntiCircleRotorAtInfinity) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor_at_infinity[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotorAtInfinity do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            anti_circle_rotor_at_infinity.group0(),
            // e15, e25, e35, e1234
            anti_circle_rotor_at_infinity.group1().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiCircleRotorOnOrigin> for DipoleInversion {
    type Error = String;
    fn try_from(anti_circle_rotor_on_origin: AntiCircleRotorOnOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotorOnOrigin do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            anti_circle_rotor_on_origin.group0().xyz(),
            // e23, e31, e12, e45
            anti_circle_rotor_on_origin.group1().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
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
        return Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(anti_dual_num[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
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
        return Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            anti_motor.group0().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            anti_motor.group1().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(anti_motor[e3215]),
        ));
    }
}

impl TryFrom<AntiMotorOnOrigin> for DipoleInversion {
    type Error = String;
    fn try_from(anti_motor_on_origin: AntiMotorOnOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_motor_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMotorOnOrigin do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            anti_motor_on_origin.group0().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiMysteryCircleRotor> for DipoleInversion {
    type Error = String;
    fn try_from(anti_mystery_circle_rotor: AntiMysteryCircleRotor) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_mystery_circle_rotor[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMysteryCircleRotor do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            anti_mystery_circle_rotor.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiVersorEvenOnOrigin> for DipoleInversion {
    type Error = String;
    fn try_from(anti_versor_even_on_origin: AntiVersorEvenOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_versor_even_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiVersorEvenOnOrigin do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            anti_versor_even_on_origin.group0().xyz(),
            // e23, e31, e12, e45
            anti_versor_even_on_origin.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(anti_versor_even_on_origin[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
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
        return Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            multi_vector.group3().xyz(),
            // e23, e31, e12, e45
            multi_vector.group5().with_w(multi_vector[e45]),
            // e15, e25, e35, e1234
            multi_vector.group4().with_w(multi_vector[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([multi_vector[e4235], multi_vector[e4315], multi_vector[e4125], multi_vector[e3215]]),
        ));
    }
}

impl TryFrom<MysteryVersorOdd> for DipoleInversion {
    type Error = String;
    fn try_from(mystery_versor_odd: MysteryVersorOdd) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_versor_odd[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryVersorOdd do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            mystery_versor_odd.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([mystery_versor_odd[e4235], mystery_versor_odd[e4315], mystery_versor_odd[e4125], 0.0]),
        ));
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
        return Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            versor_odd.group0().xyz(),
            // e23, e31, e12, e45
            versor_odd.group1(),
            // e15, e25, e35, e1234
            versor_odd.group2(),
            // e4235, e4315, e4125, e3215
            versor_odd.group3(),
        ));
    }
}

impl TryFrom<VersorOddAtInfinity> for DipoleInversion {
    type Error = String;
    fn try_from(versor_odd_at_infinity: VersorOddAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOddAtInfinity do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            versor_odd_at_infinity.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([versor_odd_at_infinity[e15], versor_odd_at_infinity[e25], versor_odd_at_infinity[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            versor_odd_at_infinity.group2(),
        ));
    }
}

impl TryFrom<VersorOddOrthogonalOrigin> for DipoleInversion {
    type Error = String;
    fn try_from(versor_odd_orthogonal_origin: VersorOddOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd_orthogonal_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOddOrthogonalOrigin do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            versor_odd_orthogonal_origin.group0().xyz(),
            // e23, e31, e12, e45
            versor_odd_orthogonal_origin.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            versor_odd_orthogonal_origin.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(versor_odd_orthogonal_origin[e3215]),
        ));
    }
}
