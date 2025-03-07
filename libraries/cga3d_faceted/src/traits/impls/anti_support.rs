// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 77
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         0       1       0
//  Maximum:         0       7       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       4       0
//  Average:         0       4       0
//  Maximum:         0      23       0
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = AntiLine;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiCircleOnOrigin {
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
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiCircleRotor {
    type Output = AntiMotor;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiCircleRotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (self.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35, e3215
            (self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0])).xyz().with_w(self[scalar]),
        )
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = AntiMotor;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiCircleRotorAligningOrigin {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (self.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35, e3215
            (self.group1() * Simd32x3::from(-1.0)).with_w(self[scalar]),
        )
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiCircleRotorAligningOriginAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ (self.group0() * Simd32x3::from(-1.0)).with_w(self[scalar]))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiCircleRotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ (self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0])).xyz().with_w(self[scalar]))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = AntiMotor;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiCircleRotorOnOrigin {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_support(self) -> Self::Output {
        let right_dual_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            right_dual_g0.xyz().with_w(0.0),
            // e15, e25, e35, e3215
            (self.group1() * Simd32x3::from(-1.0)).with_w(right_dual_g0[3]),
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
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       14        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (self.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0])).xyz().with_w(self[e4] * -1.0) * Simd32x4::from(-1.0),
            // e1, e2, e3, e5
            self.group0().with_w(self[e321] * -1.0) * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiDipoleInversionAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(
            // e235, e315, e125, e5
            (self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0])).xyz().with_w(self[e321] * -1.0) * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = AntiFlector;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiDipoleInversionOnOrigin {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn anti_support(self) -> Self::Output {
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (self.group1() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0])).yzwx() * Simd32x4::from(-1.0),
            // e1, e2, e3, e5
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiFlectorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiFlectorOnOrigin::from_groups(
            // e321, e1, e2, e3
            Simd32x4::from([self[e4] * -1.0, right_dual_g0[0], right_dual_g0[1], right_dual_g0[2]]) * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = AntiPlane;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiDipoleOnOrigin {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ self.group0().xyz().with_w(self[e321] * -1.0) * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiDualNum {
    type Output = AntiMotor;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiDualNum {
    type Output = AntiMotor;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(self[e1234]),
            // e15, e25, e35, e3215
            Simd32x3::from(0.0).with_w(self[scalar]),
        )
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiFlatOrigin {
    type Output = Infinity;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiFlatOrigin {
    type Output = Infinity;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ self[e321])
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiFlatPoint {
    type Output = Infinity;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiFlatPoint {
    type Output = Infinity;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ self[e321])
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiFlector {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiFlector {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(
            // e235, e315, e125, e5
            (self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0])).xyz().with_w(self[e321] * -1.0) * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiFlectorOnOrigin {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiFlectorOnOrigin {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_support(self) -> Self::Output {
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0().yzwx() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiLine {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiLine {
    type Output = FlatPointAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_support(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiLineOnOrigin {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiLineOnOrigin {
    type Output = FlatPointAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_support(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiMotor {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiMotor {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_support(self) -> Self::Output {
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiMotorOnOrigin {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiMotorOnOrigin {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_support(self) -> Self::Output {
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiMysteryCircleRotor {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiMysteryCircleRotor {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ (self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0])).xyz().with_w(self[scalar]))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiMysteryDipoleInversion {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiMysteryDipoleInversion {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group1().with_w(self[e321] * -1.0) * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiPlane {
    type Output = LineAtInfinity;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiPlane {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_support(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0().xyz() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiPlaneOnOrigin {
    type Output = LineAtInfinity;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiPlaneOnOrigin {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_support(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiSphereOnOrigin {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group0().xyz().with_w(self[e4] * -1.0) * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = AntiMotor;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiVersorEvenOnOrigin {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_support(self) -> Self::Output {
        let right_dual_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            right_dual_g0.xyz().with_w(right_dual_g1[3]),
            // e15, e25, e35, e3215
            right_dual_g1.xyz().with_w(right_dual_g0[3]),
        )
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
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ self.group0().with_w(self[e321] * -1.0) * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for CircleAligningOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for CircleAligningOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_support(self) -> Self::Output {
        AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for CircleAtInfinity {
    type Output = Infinity;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for CircleAtInfinity {
    type Output = Infinity;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ self[e321])
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for CircleAtOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for CircleAtOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_support(self) -> Self::Output {
        AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for CircleOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for CircleOnOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_support(self) -> Self::Output {
        AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = AntiPlane;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for CircleOrthogonalOrigin {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ self.group0().xyz().with_w(self[e321] * -1.0) * Simd32x4::from(-1.0))
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
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ self.group0().with_w(self[e321] * -1.0) * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for CircleRotorAligningOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_support(self) -> Self::Output {
        AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = Infinity;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for CircleRotorAtInfinity {
    type Output = Infinity;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ self[e321])
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for CircleRotorOnOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_support(self) -> Self::Output {
        AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0().xyz() * Simd32x3::from(-1.0))
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
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_support(self) -> Self::Output {
        AntiLine::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            (self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0])).xyz(),
        )
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for DipoleAligningOrigin {
    type Output = AntiLineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_support(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0().xyz() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for DipoleAtInfinity {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for DipoleAtInfinity {
    type Output = FlatPointAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_support(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0().xyz() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for DipoleAtOrigin {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for DipoleAtOrigin {
    type Output = AntiLineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_support(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for DipoleInversion {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for DipoleInversion {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35, scalar
            (self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0])).xyz().with_w(self[e1234]),
        )
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for DipoleInversionAligningOrigin {
    type Output = AntiMotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ (self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0])).xyz().with_w(self[e1234]))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for DipoleInversionAtInfinity {
    type Output = FlatPointAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_support(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0().xyz() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for DipoleInversionAtOrigin {
    type Output = AntiMotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for DipoleInversionOnOrigin {
    type Output = AntiMotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ (self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0])).xyz().with_w(self[e1234]))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for DipoleInversionOrthogonalOrigin {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            (self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0])).xyz(),
            // e15, e25, e35, scalar
            (self.group1() * Simd32x3::from(-1.0)).with_w(self[e1234]),
        )
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for DipoleOnOrigin {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for DipoleOnOrigin {
    type Output = AntiLineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_support(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0().xyz() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = AntiLine;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for DipoleOrthogonalOrigin {
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
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for DualNum {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for DualNum {
    type Output = AntiFlatOrigin;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e4])
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
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        3        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       23        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_dual_g9 = self.group1().wxyz() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group9().yzwx()[3], 1.0]) * Simd32x2::from([1.0, 0.0]),
            // e1, e2, e3, e4
            (self.group7() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            self[e321],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            (self.group5().with_w(self[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0])).xyz(),
            // e23, e31, e12
            self.group3().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(right_dual_g9[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            right_dual_g9.yzw() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            self.group0().yx()[1],
        )
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for MysteryCircle {
    type Output = Infinity;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for MysteryCircle {
    type Output = Infinity;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ self[e321])
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for MysteryCircleRotor {
    type Output = Infinity;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for MysteryCircleRotor {
    type Output = Infinity;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ self[e321])
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for MysteryDipole {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for MysteryDipole {
    type Output = FlatPointAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_support(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0().xyz() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for MysteryDipoleInversion {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for MysteryDipoleInversion {
    type Output = FlatPointAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_support(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0().xyz() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for MysteryVersorEven {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for MysteryVersorEven {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(
            // e235, e315, e125, e5
            (self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0])).yzw().with_w(self[e321] * -1.0) * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for MysteryVersorOdd {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for MysteryVersorOdd {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ (self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0])).xyz().with_w(self[scalar]))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for NullCircleAtOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for NullCircleAtOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_support(self) -> Self::Output {
        AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for NullDipoleAtOrigin {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for NullDipoleAtOrigin {
    type Output = AntiLineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_support(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for NullDipoleInversionAtOrigin {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for NullDipoleInversionAtOrigin {
    type Output = AntiMotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_support(self) -> Self::Output {
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for NullSphereAtOrigin {
    type Output = Scalar;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for NullSphereAtOrigin {
    type Output = Scalar;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1234])
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for NullVersorEvenAtOrigin {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for NullVersorEvenAtOrigin {
    type Output = AntiFlectorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ self.group0().xyz().with_w(self[e4] * -1.0).wxyz() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for Origin {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for Origin {
    type Output = AntiFlatOrigin;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e4])
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
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            ((self.group0().xyz().with_w(self[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0])).xyz() * Simd32x3::from(-1.0)).with_w(self[e4]),
        )
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for RoundPointAtOrigin {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for RoundPointAtOrigin {
    type Output = AntiFlatOrigin;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e4])
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for Scalar {
    type Output = Horizon;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for Scalar {
    type Output = Horizon;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[scalar])
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
impl std::ops::Div<AntiSupportPrefixOrPostfix> for SphereAtOrigin {
    type Output = Scalar;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for SphereAtOrigin {
    type Output = Scalar;
    fn anti_support(self) -> Self::Output {
        Scalar::from_groups(/* scalar */ self.group0().yx()[0])
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for SphereOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for SphereOnOrigin {
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
    //      f32        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       18        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (self.group3().xyz().with_w(self[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0])).xyz().with_w(self[e4] * -1.0) * Simd32x4::from(-1.0),
            // e1, e2, e3, e5
            (self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0])).xyz().with_w(self[e321] * -1.0) * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for VersorEvenAligningOrigin {
    type Output = AntiFlectorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiFlectorOnOrigin::from_groups(
            // e321, e1, e2, e3
            Simd32x4::from([self[e4] * -1.0, right_dual_g0[0], right_dual_g0[1], right_dual_g0[2]]) * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for VersorEvenAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(
            // e235, e315, e125, e5
            (self.group0().yzw().with_w(self[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0])).xyz().with_w(self[e321] * -1.0) * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for VersorEvenAtOrigin {
    type Output = AntiFlectorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = self.group0().xyz().with_w(self[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiFlectorOnOrigin::from_groups(
            // e321, e1, e2, e3
            Simd32x4::from([self[e4] * -1.0, right_dual_g0[0], right_dual_g0[1], right_dual_g0[2]]) * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for VersorEvenOnOrigin {
    type Output = AntiFlectorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiFlectorOnOrigin::from_groups(
            // e321, e1, e2, e3
            Simd32x4::from([self[e4] * -1.0, right_dual_g0[0], right_dual_g0[1], right_dual_g0[2]]) * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = AntiFlector;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for VersorEvenOrthogonalOrigin {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       13        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (self.group2().xyz().with_w(self[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0])).xyz().with_w(self[e4] * -1.0) * Simd32x4::from(-1.0),
            // e1, e2, e3, e5
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
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
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            right_dual_g0.xyz().with_w(self[e1234]),
            // e15, e25, e35, e3215
            (self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0])).xyz().with_w(right_dual_g0[3]),
        )
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for VersorOddAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ (self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0])).xyz().with_w(self[scalar]))
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = AntiMotor;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for VersorOddOrthogonalOrigin {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = self.group1().xyz().with_w(self[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            right_dual_g0.xyz().with_w(right_dual_g1[3]),
            // e15, e25, e35, e3215
            right_dual_g1.xyz().with_w(right_dual_g0[3]),
        )
    }
}
