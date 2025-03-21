// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 76
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
//  Average:         0       2       0       0
//  Maximum:         0       9       0       0
impl Support for AntiCircleRotor {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        SphereOnOrigin::from_groups(/* e4235, e4315, e4125, e1234 */ (self.group2().xyz() * Simd32x3::from(-1.0)).with_w(self[e45]))
    }
}
impl Support for AntiCircleRotorAligningOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn support(self) -> Self::Output {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group2().xyz() * Simd32x3::from(-1.0))
    }
}
impl Support for AntiCircleRotorAligningOriginAtInfinity {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn support(self) -> Self::Output {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group1().xyz() * Simd32x3::from(-1.0))
    }
}
impl Support for AntiCircleRotorAtInfinity {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        SphereOnOrigin::from_groups(/* e4235, e4315, e4125, e1234 */ (self.group1().xyz() * Simd32x3::from(-1.0)).with_w(self[e45]))
    }
}
impl Support for AntiDipoleInversion {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group1().xyz().with_w(self[e5] * -1.0),
            // e415, e425, e435
            self.group2().xyz(),
        )
    }
}
impl Support for AntiDipoleInversionAtInfinity {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(self[e5] * -1.0),
            // e415, e425, e435
            self.group1(),
        )
    }
}
impl Support for AntiDipoleInversionOrthogonalOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group1().with_w(self[e5] * -1.0),
            // e415, e425, e435
            self.group2().xyz(),
        )
    }
}
impl Support for AntiFlatPoint {
    type Output = LineOnOrigin;
    fn support(self) -> Self::Output {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz())
    }
}
impl Support for AntiFlector {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0().xyz().with_w(self[e5] * -1.0))
    }
}
impl Support for AntiLine {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn support(self) -> Self::Output {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group1() * Simd32x3::from(-1.0))
    }
}
impl Support for AntiMotor {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn support(self) -> Self::Output {
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group1().wxyz() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]))
    }
}
impl Support for AntiMysteryCircleRotor {
    type Output = NullSphereAtOrigin;
    fn support(self) -> Self::Output {
        use crate::elements::*;
        NullSphereAtOrigin::from_groups(/* e1234 */ self[e45])
    }
}
impl Support for AntiMysteryDipoleInversion {
    type Output = NullCircleAtOrigin;
    fn support(self) -> Self::Output {
        NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0().xyz())
    }
}
impl Support for AntiPlane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e5] * -1.0)
    }
}
impl Support for AntiScalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e12345] * -1.0)
    }
}
impl Support for Circle {
    type Output = CircleOnOrigin;
    fn support(self) -> Self::Output {
        CircleOnOrigin::from_groups(/* e423, e431, e412 */ self.group1().xyz(), /* e415, e425, e435 */ self.group2())
    }
}
impl Support for CircleAligningOrigin {
    type Output = CircleOnOrigin;
    fn support(self) -> Self::Output {
        CircleOnOrigin::from_groups(/* e423, e431, e412 */ self.group1(), /* e415, e425, e435 */ self.group2())
    }
}
impl Support for CircleAtInfinity {
    type Output = CircleOnOrigin;
    fn support(self) -> Self::Output {
        CircleOnOrigin::from_groups(/* e423, e431, e412 */ self.group0().xyz(), /* e415, e425, e435 */ self.group1())
    }
}
impl Support for CircleAtOrigin {
    type Output = LineOnOrigin;
    fn support(self) -> Self::Output {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1())
    }
}
impl Support for CircleOnOrigin {
    type Output = NullCircleAtOrigin;
    fn support(self) -> Self::Output {
        NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group1())
    }
}
impl Support for CircleOrthogonalOrigin {
    type Output = LineOnOrigin;
    fn support(self) -> Self::Output {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1())
    }
}
impl Support for CircleRotor {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e415, e425, e435, e4
            self.group2().xyz().with_w(self[e12345] * -1.0),
        )
    }
}
impl Support for CircleRotorAligningOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn support(self) -> Self::Output {
        VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group1().with_w(0.0),
            // e415, e425, e435, e4
            self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl Support for CircleRotorAligningOriginAtInfinity {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn support(self) -> Self::Output {
        VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0().with_w(0.0),
            // e415, e425, e435, e4
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl Support for CircleRotorAtInfinity {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e415, e425, e435, e4
            self.group1().xyz().with_w(self[e12345] * -1.0),
        )
    }
}
impl Support for CircleRotorOnOrigin {
    type Output = NullVersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ self.group1().with_w(self[e12345] * -1.0))
    }
}
impl Support for Dipole {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        SphereOnOrigin::from_groups(/* e4235, e4315, e4125, e1234 */ (self.group2() * Simd32x3::from(-1.0)).with_w(self[e45]))
    }
}
impl Support for DipoleAligningOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        SphereOnOrigin::from_groups(/* e4235, e4315, e4125, e1234 */ (self.group1() * Simd32x3::from(-1.0)).with_w(self[e45]))
    }
}
impl Support for DipoleAtInfinity {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        SphereOnOrigin::from_groups(/* e4235, e4315, e4125, e1234 */ (self.group1() * Simd32x3::from(-1.0)).with_w(self[e45]))
    }
}
impl Support for DipoleAtOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn support(self) -> Self::Output {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group1() * Simd32x3::from(-1.0))
    }
}
impl Support for DipoleInversion {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        7        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            self.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e45], self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0]),
        )
    }
}
impl Support for DipoleInversionAligningOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        7        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = self.group1().xyz() * Simd32x3::from(-1.0);
        DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e45], right_anti_dual_g1_xyz[0], right_anti_dual_g1_xyz[1], right_anti_dual_g1_xyz[2]]),
        )
    }
}
impl Support for DipoleInversionAtInfinity {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        7        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e45], self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0]),
        )
    }
}
impl Support for DipoleInversionAtOrigin {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e3215], self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0]))
    }
}
impl Support for DipoleInversionOnOrigin {
    type Output = NullDipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ (self.group1().yzw() * Simd32x3::from(-1.0)).with_w(self[e45]))
    }
}
impl Support for DipoleInversionOrthogonalOrigin {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e3215], self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0]))
    }
}
impl Support for DipoleOnOrigin {
    type Output = NullSphereAtOrigin;
    fn support(self) -> Self::Output {
        use crate::elements::*;
        NullSphereAtOrigin::from_groups(/* e1234 */ self[e45])
    }
}
impl Support for DipoleOrthogonalOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn support(self) -> Self::Output {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group2() * Simd32x3::from(-1.0))
    }
}
impl Support for DualNum {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e12345] * -1.0)
    }
}
impl Support for FlatOrigin {
    type Output = NullSphereAtOrigin;
    fn support(self) -> Self::Output {
        use crate::elements::*;
        NullSphereAtOrigin::from_groups(/* e1234 */ self[e45])
    }
}
impl Support for FlatPoint {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn support(self) -> Self::Output {
        SphereOnOrigin::from_groups(/* e4235, e4315, e4125, e1234 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl Support for FlatPointAtInfinity {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn support(self) -> Self::Output {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl Support for Flector {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn support(self) -> Self::Output {
        DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1234, e4235, e4315, e4125
            self.group0().wxyz() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
        )
    }
}
impl Support for FlectorAtInfinity {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn support(self) -> Self::Output {
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0().wxyz() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]))
    }
}
impl Support for FlectorOnOrigin {
    type Output = NullDipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn support(self) -> Self::Output {
        NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ self.group0().yzwx() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl Support for Horizon {
    type Output = FlatOrigin;
    fn support(self) -> Self::Output {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ self[e3215])
    }
}
impl Support for Infinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e5] * -1.0)
    }
}
impl Support for Line {
    type Output = CircleOnOrigin;
    fn support(self) -> Self::Output {
        CircleOnOrigin::from_groups(/* e423, e431, e412 */ self.group0(), /* e415, e425, e435 */ self.group1())
    }
}
impl Support for LineAtInfinity {
    type Output = LineOnOrigin;
    fn support(self) -> Self::Output {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0())
    }
}
impl Support for LineOnOrigin {
    type Output = NullCircleAtOrigin;
    fn support(self) -> Self::Output {
        NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0())
    }
}
impl Support for Motor {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(self[e5] * -1.0),
            // e415, e425, e435, e4
            self.group1().xyz().with_w(self[e12345] * -1.0),
        )
    }
}
impl Support for MotorAtInfinity {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn support(self) -> Self::Output {
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
    }
}
impl Support for MotorOnOrigin {
    type Output = NullVersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn support(self) -> Self::Output {
        NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
    }
}
impl Support for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0        9        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1 = self.group9().yzwx() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e5] * -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e12345] * -1.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([right_anti_dual_g1[0], right_anti_dual_g1[1], right_anti_dual_g1[2], self[e3215]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group8().with_w(0.0),
            // e423, e431, e412
            self.group6().xyz(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e45], self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0]),
            // e3215
            0.0,
        )
    }
}
impl Support for MysteryCircle {
    type Output = NullCircleAtOrigin;
    fn support(self) -> Self::Output {
        NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0().xyz())
    }
}
impl Support for MysteryCircleRotor {
    type Output = NullVersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ self.group0().xyz().with_w(self[e12345] * -1.0))
    }
}
impl Support for MysteryDipole {
    type Output = NullSphereAtOrigin;
    fn support(self) -> Self::Output {
        use crate::elements::*;
        NullSphereAtOrigin::from_groups(/* e1234 */ self[e45])
    }
}
impl Support for MysteryDipoleInversion {
    type Output = NullDipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ (self.group1() * Simd32x3::from(-1.0)).with_w(self[e45]))
    }
}
impl Support for MysteryVersorEven {
    type Output = NullVersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ self.group1().xyz().with_w(self[e12345] * -1.0))
    }
}
impl Support for MysteryVersorOdd {
    type Output = NullDipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ (self.group0().yzw() * Simd32x3::from(-1.0)).with_w(self[e45]))
    }
}
impl Support for Plane {
    type Output = DipoleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn support(self) -> Self::Output {
        DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl Support for PlaneOnOrigin {
    type Output = NullDipoleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn support(self) -> Self::Output {
        NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl Support for RoundPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e5] * -1.0)
    }
}
impl Support for RoundPointAtOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e5] * -1.0)
    }
}
impl Support for Sphere {
    type Output = DipoleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn support(self) -> Self::Output {
        DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl Support for SphereAtOrigin {
    type Output = FlatOrigin;
    fn support(self) -> Self::Output {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ self[e3215])
    }
}
impl Support for SphereOnOrigin {
    type Output = NullDipoleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn support(self) -> Self::Output {
        NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0().xyz() * Simd32x3::from(-1.0))
    }
}
impl Support for VersorEven {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group1().xyz().with_w(self[e5] * -1.0),
            // e415, e425, e435, e4
            self.group2().xyz().with_w(self[e12345] * -1.0),
        )
    }
}
impl Support for VersorEvenAligningOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group1().xyz().with_w(self[e5] * -1.0),
            // e415, e425, e435, e4
            self.group2().xyz().with_w(self[e12345] * -1.0),
        )
    }
}
impl Support for VersorEvenAtInfinity {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group1().xyz().with_w(self[e5] * -1.0),
            // e415, e425, e435, e4
            self.group2().xyz().with_w(self[e12345] * -1.0),
        )
    }
}
impl Support for VersorEvenAtOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn support(self) -> Self::Output {
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
    }
}
impl Support for VersorEvenOnOrigin {
    type Output = NullVersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ self.group1().xyz().with_w(self[e12345] * -1.0))
    }
}
impl Support for VersorEvenOrthogonalOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn support(self) -> Self::Output {
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
    }
}
impl Support for VersorOdd {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        7        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2_xyz = self.group2().xyz() * Simd32x3::from(-1.0);
        DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            self.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e45], right_anti_dual_g2_xyz[0], right_anti_dual_g2_xyz[1], right_anti_dual_g2_xyz[2]]),
        )
    }
}
impl Support for VersorOddAtInfinity {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        7        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2_xyz = self.group0().yzw() * Simd32x3::from(-1.0);
        DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e45], right_anti_dual_g2_xyz[0], right_anti_dual_g2_xyz[1], right_anti_dual_g2_xyz[2]]),
        )
    }
}
impl Support for VersorOddOrthogonalOrigin {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e3215], self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0]))
    }
}
