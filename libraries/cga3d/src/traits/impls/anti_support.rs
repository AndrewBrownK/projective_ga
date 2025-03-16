// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 18
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         0       1       0
//  Maximum:         0       4       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       3       0
//  Average:         0       3       0
//  Maximum:         0      12       0
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiCircleRotor {
    type Output = AntiMotor;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiCircleRotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (self.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35, e3215
            Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
        )
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiDipoleInversion {
    type Output = AntiFlector;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiDipoleInversion {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (self.group3().xyz() * Simd32x3::from(-1.0)).with_w(self[e4]),
            // e1, e2, e3, e5
            (self.group0() * Simd32x3::from(-1.0)).with_w(self[e321]),
        )
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiDualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl std::ops::DivAssign<AntiSupportPrefixOrPostfix> for AntiDualNum {
    fn div_assign(&mut self, _rhs: AntiSupportPrefixOrPostfix) {
        *self = self.anti_support()
    }
}
impl AntiSupport for AntiDualNum {
    type Output = AntiDualNum;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([self[scalar], 0.0]))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiFlatPoint {
    type Output = DualNum;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiFlatPoint {
    type Output = DualNum;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([self[e321], 0.0]))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiFlector {
    type Output = AntiFlector;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl std::ops::DivAssign<AntiSupportPrefixOrPostfix> for AntiFlector {
    fn div_assign(&mut self, _rhs: AntiSupportPrefixOrPostfix) {
        *self = self.anti_support()
    }
}
impl AntiSupport for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (self.group1().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(self[e321]),
        )
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiLine {
    type Output = FlatPoint;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiLine {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_support(self) -> Self::Output {
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ (self.group0() * Simd32x3::from(-1.0)).with_w(0.0))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiMotor {
    type Output = Flector;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiMotor {
    type Output = Flector;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            self.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[scalar]),
        )
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiPlane {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiPlane {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_support(self) -> Self::Output {
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for Circle {
    type Output = AntiPlane;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for Circle {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ (self.group0() * Simd32x3::from(-1.0)).with_w(self[e321]))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for CircleRotor {
    type Output = AntiPlane;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for CircleRotor {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ (self.group0() * Simd32x3::from(-1.0)).with_w(self[e321]))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for Dipole {
    type Output = AntiLine;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for Dipole {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_support(self) -> Self::Output {
        AntiLine::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1().xyz() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for DipoleInversion {
    type Output = AntiMotor;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for DipoleInversion {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (self.group0() * Simd32x3::from(-1.0)).with_w(self[e1234]),
            // e15, e25, e35, e3215
            self.group1().xyz().with_w(0.0),
        )
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl std::ops::DivAssign<AntiSupportPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: AntiSupportPrefixOrPostfix) {
        *self = self.anti_support()
    }
}
impl AntiSupport for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        4        0
    // no simd        0       12        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[e1234], 0.0]),
            // e1, e2, e3, e4
            (self.group7() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            self[e321],
            // e15, e25, e35, e45
            (self.group5() * Simd32x3::from(-1.0)).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group4() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e4]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self.group0().yx()[1]),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for RoundPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for RoundPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e4]))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for Scalar {
    type Output = AntiDualNum;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for Scalar {
    type Output = AntiDualNum;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([self[scalar], 0.0]))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for Sphere {
    type Output = Scalar;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for Sphere {
    type Output = Scalar;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1234])
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for VersorEven {
    type Output = AntiFlector;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for VersorEven {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e321]),
        )
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for VersorOdd {
    type Output = AntiMotor;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for VersorOdd {
    type Output = AntiMotor;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]),
            // e15, e25, e35, e3215
            Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
        )
    }
}
