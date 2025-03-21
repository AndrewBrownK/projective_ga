// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 77
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         0       1       0     N/A
//  Average:         0       1       0     N/A
//  Maximum:         0       6       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         0       3       0       0
//  Average:         0       3       0       0
//  Maximum:         0      17       0       0
impl AntiSupport for AntiCircleOnOrigin {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn anti_support(self) -> Self::Output {
        AntiLine::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl AntiSupport for AntiCircleRotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (self.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35, e3215
            (self.group1().xyz() * Simd32x3::from(-1.0)).with_w(self[scalar]),
        )
    }
}
impl AntiSupport for AntiCircleRotorAligningOrigin {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
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
impl AntiSupport for AntiCircleRotorAligningOriginAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ (self.group0() * Simd32x3::from(-1.0)).with_w(self[scalar]))
    }
}
impl AntiSupport for AntiCircleRotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[scalar]))
    }
}
impl AntiSupport for AntiCircleRotorOnOrigin {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35, e3215
            (self.group1() * Simd32x3::from(-1.0)).with_w(self[scalar]),
        )
    }
}
impl AntiSupport for AntiDipoleInversion {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
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
impl AntiSupport for AntiDipoleInversionAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ (self.group2().xyz() * Simd32x3::from(-1.0)).with_w(self[e321]))
    }
}
impl AntiSupport for AntiDipoleInversionOnOrigin {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn anti_support(self) -> Self::Output {
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group1().yzwx() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl AntiSupport for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiFlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = self.group0().xyz();
        AntiFlectorOnOrigin::from_groups(
            // e321, e1, e2, e3
            Simd32x4::from([self[e4], right_dual_g0_xyz[0] * -1.0, right_dual_g0_xyz[1] * -1.0, right_dual_g0_xyz[2] * -1.0]),
        )
    }
}
impl AntiSupport for AntiDipoleOnOrigin {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_support(self) -> Self::Output {
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
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
impl AntiSupport for AntiFlatOrigin {
    type Output = Infinity;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ self[e321])
    }
}
impl AntiSupport for AntiFlatPoint {
    type Output = Infinity;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ self[e321])
    }
}
impl AntiSupport for AntiFlector {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ (self.group1().xyz() * Simd32x3::from(-1.0)).with_w(self[e321]))
    }
}
impl AntiSupport for AntiFlectorOnOrigin {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_support(self) -> Self::Output {
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0().yzwx() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl AntiSupport for AntiLine {
    type Output = FlatPointAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl AntiSupport for AntiLineOnOrigin {
    type Output = FlatPointAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl AntiSupport for AntiMotor {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_support(self) -> Self::Output {
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl AntiSupport for AntiMotorOnOrigin {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_support(self) -> Self::Output {
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl AntiSupport for AntiMysteryCircleRotor {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[scalar]))
    }
}
impl AntiSupport for AntiMysteryDipoleInversion {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ (self.group1() * Simd32x3::from(-1.0)).with_w(self[e321]))
    }
}
impl AntiSupport for AntiPlane {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0().xyz() * Simd32x3::from(-1.0))
    }
}
impl AntiSupport for AntiPlaneOnOrigin {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl AntiSupport for AntiSphereOnOrigin {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_support(self) -> Self::Output {
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl AntiSupport for AntiVersorEvenOnOrigin {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e1234]),
            // e15, e25, e35, e3215
            (self.group1().xyz() * Simd32x3::from(-1.0)).with_w(self[scalar]),
        )
    }
}
impl AntiSupport for Circle {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ (self.group0() * Simd32x3::from(-1.0)).with_w(self[e321]))
    }
}
impl AntiSupport for CircleAligningOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl AntiSupport for CircleAtInfinity {
    type Output = Infinity;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ self[e321])
    }
}
impl AntiSupport for CircleAtOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl AntiSupport for CircleOnOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl AntiSupport for CircleOrthogonalOrigin {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_support(self) -> Self::Output {
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl AntiSupport for CircleRotor {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ (self.group0() * Simd32x3::from(-1.0)).with_w(self[e321]))
    }
}
impl AntiSupport for CircleRotorAligningOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl AntiSupport for CircleRotorAtInfinity {
    type Output = Infinity;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ self[e321])
    }
}
impl AntiSupport for CircleRotorOnOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0().xyz() * Simd32x3::from(-1.0))
    }
}
impl AntiSupport for Dipole {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn anti_support(self) -> Self::Output {
        AntiLine::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1().xyz() * Simd32x3::from(-1.0),
        )
    }
}
impl AntiSupport for DipoleAligningOrigin {
    type Output = AntiLineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0().xyz() * Simd32x3::from(-1.0))
    }
}
impl AntiSupport for DipoleAtInfinity {
    type Output = FlatPointAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0().xyz() * Simd32x3::from(-1.0))
    }
}
impl AntiSupport for DipoleAtOrigin {
    type Output = AntiLineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl AntiSupport for DipoleInversion {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35, scalar
            (self.group1().xyz() * Simd32x3::from(-1.0)).with_w(self[e1234]),
        )
    }
}
impl AntiSupport for DipoleInversionAligningOrigin {
    type Output = AntiMotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e1234]))
    }
}
impl AntiSupport for DipoleInversionAtInfinity {
    type Output = FlatPointAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0().xyz() * Simd32x3::from(-1.0))
    }
}
impl AntiSupport for DipoleInversionAtOrigin {
    type Output = AntiMotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e1234]))
    }
}
impl AntiSupport for DipoleInversionOnOrigin {
    type Output = AntiMotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e1234]))
    }
}
impl AntiSupport for DipoleInversionOrthogonalOrigin {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            self.group0().xyz() * Simd32x3::from(-1.0),
            // e15, e25, e35, scalar
            (self.group1() * Simd32x3::from(-1.0)).with_w(self[e1234]),
        )
    }
}
impl AntiSupport for DipoleOnOrigin {
    type Output = AntiLineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0().xyz() * Simd32x3::from(-1.0))
    }
}
impl AntiSupport for DipoleOrthogonalOrigin {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn anti_support(self) -> Self::Output {
        AntiLine::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl AntiSupport for DualNum {
    type Output = AntiFlatOrigin;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e4])
    }
}
impl AntiSupport for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        4        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       17        0        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_dual_g9 = self.group1().wxyz() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[e1234], 0.0]),
            // e1, e2, e3, e4
            (self.group7() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            self[e321],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group5() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group3().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(right_dual_g9[0] * -1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            right_dual_g9.yzw() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            self[scalar],
        )
    }
}
impl AntiSupport for MysteryCircle {
    type Output = Infinity;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ self[e321])
    }
}
impl AntiSupport for MysteryCircleRotor {
    type Output = Infinity;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ self[e321])
    }
}
impl AntiSupport for MysteryDipole {
    type Output = FlatPointAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0().xyz() * Simd32x3::from(-1.0))
    }
}
impl AntiSupport for MysteryDipoleInversion {
    type Output = FlatPointAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0().xyz() * Simd32x3::from(-1.0))
    }
}
impl AntiSupport for MysteryVersorEven {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        7        0        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(
            // e235, e315, e125, e5
            ((self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0])).yzw() * Simd32x3::from(-1.0)).with_w(self[e321]),
        )
    }
}
impl AntiSupport for MysteryVersorOdd {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ (self.group1().xyz() * Simd32x3::from(-1.0)).with_w(self[scalar]))
    }
}
impl AntiSupport for NullCircleAtOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl AntiSupport for NullDipoleAtOrigin {
    type Output = AntiLineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl AntiSupport for NullDipoleInversionAtOrigin {
    type Output = AntiMotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_support(self) -> Self::Output {
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl AntiSupport for NullSphereAtOrigin {
    type Output = Scalar;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1234])
    }
}
impl AntiSupport for NullVersorEvenAtOrigin {
    type Output = AntiFlectorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_support(self) -> Self::Output {
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ self.group0().wxyz() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]))
    }
}
impl AntiSupport for Origin {
    type Output = AntiFlatOrigin;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e4])
    }
}
impl AntiSupport for RoundPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e4]))
    }
}
impl AntiSupport for RoundPointAtOrigin {
    type Output = AntiFlatOrigin;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e4])
    }
}
impl AntiSupport for Scalar {
    type Output = Horizon;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[scalar])
    }
}
impl AntiSupport for Sphere {
    type Output = Scalar;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1234])
    }
}
impl AntiSupport for SphereAtOrigin {
    type Output = Scalar;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1234])
    }
}
impl AntiSupport for SphereOnOrigin {
    type Output = Scalar;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1234])
    }
}
impl AntiSupport for VersorEven {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        7        0        0
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
impl AntiSupport for VersorEvenAligningOrigin {
    type Output = AntiFlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = self.group0().xyz();
        AntiFlectorOnOrigin::from_groups(
            // e321, e1, e2, e3
            Simd32x4::from([self[e4], right_dual_g0_xyz[0] * -1.0, right_dual_g0_xyz[1] * -1.0, right_dual_g0_xyz[2] * -1.0]),
        )
    }
}
impl AntiSupport for VersorEvenAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ (self.group0().yzw() * Simd32x3::from(-1.0)).with_w(self[e321]))
    }
}
impl AntiSupport for VersorEvenAtOrigin {
    type Output = AntiFlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = self.group0().xyz();
        AntiFlectorOnOrigin::from_groups(
            // e321, e1, e2, e3
            Simd32x4::from([self[e4], right_dual_g0_xyz[0] * -1.0, right_dual_g0_xyz[1] * -1.0, right_dual_g0_xyz[2] * -1.0]),
        )
    }
}
impl AntiSupport for VersorEvenOnOrigin {
    type Output = AntiFlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = self.group0().xyz();
        AntiFlectorOnOrigin::from_groups(
            // e321, e1, e2, e3
            Simd32x4::from([self[e4], right_dual_g0_xyz[0] * -1.0, right_dual_g0_xyz[1] * -1.0, right_dual_g0_xyz[2] * -1.0]),
        )
    }
}
impl AntiSupport for VersorEvenOrthogonalOrigin {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn anti_support(self) -> Self::Output {
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl AntiSupport for VersorOdd {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e1234]),
            // e15, e25, e35, e3215
            (self.group1().xyz() * Simd32x3::from(-1.0)).with_w(self[scalar]),
        )
    }
}
impl AntiSupport for VersorOddAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ (self.group1().xyz() * Simd32x3::from(-1.0)).with_w(self[scalar]))
    }
}
impl AntiSupport for VersorOddOrthogonalOrigin {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e1234]),
            // e15, e25, e35, e3215
            (self.group1().xyz() * Simd32x3::from(-1.0)).with_w(self[scalar]),
        )
    }
}
