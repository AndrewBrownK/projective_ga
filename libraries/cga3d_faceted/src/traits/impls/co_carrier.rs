// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 77
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         0       0       0
//  Maximum:         0       4       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         0       1       0
//  Maximum:         0      10       0
impl CoCarrier for AntiCircleOnOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn co_carrier(self) -> Self::Output {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl CoCarrier for AntiCircleRotor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ (self.group0() * Simd32x3::from(-1.0)).with_w(self[e45]))
    }
}
impl CoCarrier for AntiCircleRotorAligningOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn co_carrier(self) -> Self::Output {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl CoCarrier for AntiCircleRotorAtInfinity {
    type Output = Horizon;
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[e45])
    }
}
impl CoCarrier for AntiCircleRotorOnOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn co_carrier(self) -> Self::Output {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz() * Simd32x3::from(-1.0))
    }
}
impl CoCarrier for AntiDipoleInversion {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            self.group0(),
            // e235, e315, e125, e12345
            self.group1().xyz().with_w(self[e4] * -1.0),
        )
    }
}
impl CoCarrier for AntiDipoleInversionAtInfinity {
    type Output = LineAtInfinity;
    fn co_carrier(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0().xyz())
    }
}
impl CoCarrier for AntiDipoleInversionOnOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0().xyz().with_w(self[e4] * -1.0))
    }
}
impl CoCarrier for AntiDipoleInversionOrthogonalOrigin {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            self.group0().xyz(),
            // e235, e315, e125, e12345
            self.group1().with_w(self[e4] * -1.0),
        )
    }
}
impl CoCarrier for AntiDipoleOnOrigin {
    type Output = LineOnOrigin;
    fn co_carrier(self) -> Self::Output {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz())
    }
}
impl CoCarrier for AntiDualNum {
    type Output = FlatOrigin;
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ self[e1234])
    }
}
impl CoCarrier for AntiMysteryCircleRotor {
    type Output = Horizon;
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[e45])
    }
}
impl CoCarrier for AntiMysteryDipoleInversion {
    type Output = LineAtInfinity;
    fn co_carrier(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0().xyz())
    }
}
impl CoCarrier for AntiScalar {
    type Output = Infinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ self[e12345] * -1.0)
    }
}
impl CoCarrier for AntiSphereOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e4] * -1.0)
    }
}
impl CoCarrier for AntiVersorEvenOnOrigin {
    type Output = FlectorOnOrigin;
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e1234], self[e41], self[e42], self[e43]]))
    }
}
impl CoCarrier for Circle {
    type Output = Line;
    fn co_carrier(self) -> Self::Output {
        Line::from_groups(/* e415, e425, e435 */ self.group0(), /* e235, e315, e125 */ self.group1().xyz())
    }
}
impl CoCarrier for CircleAligningOrigin {
    type Output = Line;
    fn co_carrier(self) -> Self::Output {
        Line::from_groups(/* e415, e425, e435 */ self.group0(), /* e235, e315, e125 */ self.group1())
    }
}
impl CoCarrier for CircleAtInfinity {
    type Output = LineAtInfinity;
    fn co_carrier(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0().xyz())
    }
}
impl CoCarrier for CircleAtOrigin {
    type Output = LineOnOrigin;
    fn co_carrier(self) -> Self::Output {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0())
    }
}
impl CoCarrier for CircleOnOrigin {
    type Output = Line;
    fn co_carrier(self) -> Self::Output {
        Line::from_groups(/* e415, e425, e435 */ self.group0(), /* e235, e315, e125 */ self.group1())
    }
}
impl CoCarrier for CircleOrthogonalOrigin {
    type Output = LineOnOrigin;
    fn co_carrier(self) -> Self::Output {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz())
    }
}
impl CoCarrier for CircleRotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().with_w(0.0),
            // e235, e315, e125, e5
            self.group1().xyz().with_w(self[e12345] * -1.0),
        )
    }
}
impl CoCarrier for CircleRotorAligningOrigin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().with_w(0.0),
            // e235, e315, e125, e5
            self.group1().with_w(self[e12345] * -1.0),
        )
    }
}
impl CoCarrier for CircleRotorAligningOriginAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0().with_w(self[e12345] * -1.0))
    }
}
impl CoCarrier for CircleRotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0().xyz().with_w(self[e12345] * -1.0))
    }
}
impl CoCarrier for CircleRotorOnOrigin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            self.group1().with_w(self[e12345] * -1.0),
        )
    }
}
impl CoCarrier for Dipole {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ (self.group0() * Simd32x3::from(-1.0)).with_w(self[e45]))
    }
}
impl CoCarrier for DipoleAligningOrigin {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn co_carrier(self) -> Self::Output {
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl CoCarrier for DipoleAtInfinity {
    type Output = Horizon;
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[e45])
    }
}
impl CoCarrier for DipoleAtOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn co_carrier(self) -> Self::Output {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl CoCarrier for DipoleInversion {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e1234]]),
            // e4235, e4315, e4125, e3215
            (self.group0() * Simd32x3::from(-1.0)).with_w(self[e45]),
        )
    }
}
impl CoCarrier for DipoleInversionAligningOrigin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            (self.group2().xyz() * Simd32x3::from(-1.0)).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl CoCarrier for DipoleInversionAtInfinity {
    type Output = FlectorAtInfinity;
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e45]]))
    }
}
impl CoCarrier for DipoleInversionAtOrigin {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        FlectorOnOrigin::from_groups(
            // e45, e4235, e4315, e4125
            Simd32x4::from([self[e1234], self[e41], self[e42], self[e43]]) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
        )
    }
}
impl CoCarrier for DipoleInversionOnOrigin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn co_carrier(self) -> Self::Output {
        Flector::from_groups(
            // e15, e25, e35, e45
            self.group1().yzwx() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl CoCarrier for DipoleInversionOrthogonalOrigin {
    type Output = FlectorOnOrigin;
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e1234], self[e41], self[e42], self[e43]]))
    }
}
impl CoCarrier for DipoleOnOrigin {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn co_carrier(self) -> Self::Output {
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl CoCarrier for DipoleOrthogonalOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn co_carrier(self) -> Self::Output {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl CoCarrier for DualNum {
    type Output = Motor;
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(0.0).with_w(self[e4]),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(self[e12345]),
        )
    }
}
impl CoCarrier for FlatOrigin {
    type Output = Horizon;
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[e45])
    }
}
impl CoCarrier for FlatPoint {
    type Output = Horizon;
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[e45])
    }
}
impl CoCarrier for Flector {
    type Output = FlectorAtInfinity;
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e45]]))
    }
}
impl CoCarrier for FlectorOnOrigin {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn co_carrier(self) -> Self::Output {
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group0().yzwx() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl CoCarrier for Line {
    type Output = LineAtInfinity;
    fn co_carrier(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0())
    }
}
impl CoCarrier for LineOnOrigin {
    type Output = LineAtInfinity;
    fn co_carrier(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0())
    }
}
impl CoCarrier for Motor {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0().xyz().with_w(self[e12345] * -1.0))
    }
}
impl CoCarrier for MotorOnOrigin {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0().xyz().with_w(self[e12345] * -1.0))
    }
}
impl CoCarrier for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1 = self.group9().yzwx() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_anti_dual_g7 = self.group3().xyz() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group1().wxyz()[0]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0().yx()[0] * -1.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(right_anti_dual_g1[3]),
            // e15, e25, e35
            right_anti_dual_g1.xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group7().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group6().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, right_anti_dual_g7[0], right_anti_dual_g7[1], right_anti_dual_g7[2]]),
            // e3215
            self[e45],
        )
    }
}
impl CoCarrier for MysteryCircle {
    type Output = LineAtInfinity;
    fn co_carrier(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0().xyz())
    }
}
impl CoCarrier for MysteryCircleRotor {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0().xyz().with_w(self[e12345] * -1.0))
    }
}
impl CoCarrier for MysteryDipole {
    type Output = Horizon;
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[e45])
    }
}
impl CoCarrier for MysteryDipoleInversion {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ (self.group1() * Simd32x3::from(-1.0)).with_w(self[e45]))
    }
}
impl CoCarrier for MysteryVersorEven {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group1().xyz().with_w(self[e12345] * -1.0))
    }
}
impl CoCarrier for MysteryVersorOdd {
    type Output = FlectorAtInfinity;
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e45]]))
    }
}
impl CoCarrier for NullCircleAtOrigin {
    type Output = LineOnOrigin;
    fn co_carrier(self) -> Self::Output {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0())
    }
}
impl CoCarrier for NullDipoleAtOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn co_carrier(self) -> Self::Output {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl CoCarrier for NullDipoleInversionAtOrigin {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn co_carrier(self) -> Self::Output {
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0().wxyz() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]))
    }
}
impl CoCarrier for NullSphereAtOrigin {
    type Output = FlatOrigin;
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ self[e1234])
    }
}
impl CoCarrier for NullVersorEvenAtOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0().xyz().with_w(self[e4] * -1.0))
    }
}
impl CoCarrier for Origin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e4] * -1.0)
    }
}
impl CoCarrier for Plane {
    type Output = FlatPointAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn co_carrier(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0().xyz() * Simd32x3::from(-1.0))
    }
}
impl CoCarrier for PlaneOnOrigin {
    type Output = FlatPointAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn co_carrier(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl CoCarrier for RoundPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e4] * -1.0)
    }
}
impl CoCarrier for RoundPointAtOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e4] * -1.0)
    }
}
impl CoCarrier for Sphere {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e1234]))
    }
}
impl CoCarrier for SphereAtOrigin {
    type Output = FlatOrigin;
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ self[e1234])
    }
}
impl CoCarrier for SphereOnOrigin {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn co_carrier(self) -> Self::Output {
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl CoCarrier for VersorEven {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().xyz().with_w(self[e4] * -1.0),
            // e235, e315, e125, e5
            self.group1().xyz().with_w(self[e12345] * -1.0),
        )
    }
}
impl CoCarrier for VersorEvenAligningOrigin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().xyz().with_w(self[e4] * -1.0),
            // e235, e315, e125, e5
            self.group1().xyz().with_w(self[e12345] * -1.0),
        )
    }
}
impl CoCarrier for VersorEvenAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group1().xyz().with_w(self[e12345] * -1.0))
    }
}
impl CoCarrier for VersorEvenAtOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0().xyz().with_w(self[e4] * -1.0))
    }
}
impl CoCarrier for VersorEvenOnOrigin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().xyz().with_w(self[e4] * -1.0),
            // e235, e315, e125, e5
            self.group1().xyz().with_w(self[e12345] * -1.0),
        )
    }
}
impl CoCarrier for VersorEvenOrthogonalOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0().xyz().with_w(self[e4] * -1.0))
    }
}
impl CoCarrier for VersorOdd {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            (self.group3().xyz() * Simd32x3::from(-1.0)).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45]]),
        )
    }
}
impl CoCarrier for VersorOddAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(
            // e15, e25, e35, e3215
            Simd32x4::from([self[e4235] * -1.0, self[e4315] * -1.0, self[e4125], self[e45]]) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]),
        )
    }
}
impl CoCarrier for VersorOddOrthogonalOrigin {
    type Output = FlectorOnOrigin;
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e1234], self[e41], self[e42], self[e43]]))
    }
}
