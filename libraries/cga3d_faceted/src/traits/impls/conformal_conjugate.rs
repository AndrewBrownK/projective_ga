// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 95
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         0       1       0
//  Maximum:         0       8       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       4       0
//  Average:         0       3       0
//  Maximum:         0      19       0
impl ConformalConjugate for AntiCircleOnOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl ConformalConjugate for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e45] * -1.0),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl ConformalConjugate for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl ConformalConjugate for AntiCircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            self.group0(),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl ConformalConjugate for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0().xyz().with_w(self[e45] * -1.0),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl ConformalConjugate for AntiCircleRotorOnOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl ConformalConjugate for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            self.group3().xyz().with_w(self[e5] * -1.0),
        )
    }
}
impl ConformalConjugate for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
            // e1, e2, e3, e5
            self.group2().xyz().with_w(self[e5] * -1.0),
        )
    }
}
impl ConformalConjugate for AntiDipoleInversionOnOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl ConformalConjugate for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0().xyz().with_w(self[e5] * -1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl ConformalConjugate for AntiDipoleOnOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl ConformalConjugate for AntiDualNum {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl ConformalConjugate for AntiFlatOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl ConformalConjugate for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl ConformalConjugate for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            self.group1().xyz().with_w(self[e5] * -1.0),
        )
    }
}
impl ConformalConjugate for AntiFlectorOnOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl ConformalConjugate for AntiLine {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conformal_conjugate(self) -> Self {
        AntiLine::from_groups(/* e23, e31, e12 */ self.group0(), /* e15, e25, e35 */ self.group1() * Simd32x3::from(-1.0))
    }
}
impl ConformalConjugate for AntiLineOnOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl ConformalConjugate for AntiMotor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        AntiMotor::from_groups(/* e23, e31, e12, scalar */ self.group0(), /* e15, e25, e35, e3215 */ self.group1() * Simd32x4::from(-1.0))
    }
}
impl ConformalConjugate for AntiMotorOnOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl ConformalConjugate for AntiMysteryCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        AntiMysteryCircleRotor::from_groups(/* e23, e31, e12, e45 */ self.group0().xyz().with_w(self[e45] * -1.0), /* scalar */ self[scalar])
    }
}
impl ConformalConjugate for AntiMysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        AntiMysteryDipoleInversion::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3
            self.group1(),
        )
    }
}
impl ConformalConjugate for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ self.group0().xyz().with_w(self[e5] * -1.0))
    }
}
impl ConformalConjugate for AntiPlaneOnOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl ConformalConjugate for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e12345] * -1.0)
    }
}
impl ConformalConjugate for AntiSphereOnOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl ConformalConjugate for AntiVersorEvenOnOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl ConformalConjugate for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conformal_conjugate(self) -> Self {
        Circle::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            self.group2() * Simd32x3::from(-1.0),
        )
    }
}
impl ConformalConjugate for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn conformal_conjugate(self) -> Self {
        CircleAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group2() * Simd32x3::from(-1.0),
        )
    }
}
impl ConformalConjugate for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conformal_conjugate(self) -> Self {
        CircleAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl ConformalConjugate for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conformal_conjugate(self) -> Self {
        CircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0(), /* e235, e315, e125 */ self.group1() * Simd32x3::from(-1.0))
    }
}
impl ConformalConjugate for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conformal_conjugate(self) -> Self {
        CircleOnOrigin::from_groups(/* e423, e431, e412 */ self.group0(), /* e415, e425, e435 */ self.group1() * Simd32x3::from(-1.0))
    }
}
impl ConformalConjugate for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conformal_conjugate(self) -> Self {
        CircleOrthogonalOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0(), /* e235, e315, e125 */ self.group1() * Simd32x3::from(-1.0))
    }
}
impl ConformalConjugate for CircleRotor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl ConformalConjugate for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conformal_conjugate(self) -> Self {
        CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl ConformalConjugate for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conformal_conjugate(self) -> Self {
        CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl ConformalConjugate for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        CircleRotorAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl ConformalConjugate for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(self[e12345] * -1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl ConformalConjugate for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e45] * -1.0),
            // e15, e25, e35
            self.group2() * Simd32x3::from(-1.0),
        )
    }
}
impl ConformalConjugate for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0().xyz().with_w(self[e45] * -1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl ConformalConjugate for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0().xyz().with_w(self[e45] * -1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl ConformalConjugate for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conformal_conjugate(self) -> Self {
        DipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0(), /* e15, e25, e35 */ self.group1() * Simd32x3::from(-1.0))
    }
}
impl ConformalConjugate for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e45] * -1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group3() * Simd32x4::from(-1.0),
        )
    }
}
impl ConformalConjugate for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0().xyz().with_w(self[e45] * -1.0),
            // e15, e25, e35, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl ConformalConjugate for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0().xyz().with_w(self[e45] * -1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl ConformalConjugate for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0().xyz().with_w(self[e3215] * -1.0),
            // e15, e25, e35, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl ConformalConjugate for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0().xyz().with_w(self[e45] * -1.0),
            // e1234, e4235, e4315, e4125
            self.group1() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
        )
    }
}
impl ConformalConjugate for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0().xyz().with_w(self[e3215] * -1.0),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl ConformalConjugate for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0().xyz().with_w(self[e45] * -1.0))
    }
}
impl ConformalConjugate for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conformal_conjugate(self) -> Self {
        DipoleOrthogonalOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35
            self.group2() * Simd32x3::from(-1.0),
        )
    }
}
impl ConformalConjugate for DualNum {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn conformal_conjugate(self) -> Self {
        DualNum::from_groups(/* e4, e12345 */ self.group0() * Simd32x2::from([1.0, -1.0]))
    }
}
impl ConformalConjugate for FlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ self[e45] * -1.0)
    }
}
impl ConformalConjugate for FlatPoint {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl ConformalConjugate for FlatPointAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conformal_conjugate(self) -> Self {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl ConformalConjugate for Flector {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        Flector::from_groups(
            // e15, e25, e35, e45
            self.group0() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl ConformalConjugate for FlectorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl ConformalConjugate for FlectorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl ConformalConjugate for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[e3215] * -1.0)
    }
}
impl ConformalConjugate for Infinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ self[e5] * -1.0)
    }
}
impl ConformalConjugate for Line {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn conformal_conjugate(self) -> Self {
        Line::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl ConformalConjugate for LineAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conformal_conjugate(self) -> Self {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl ConformalConjugate for LineOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conformal_conjugate(self) -> Self {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl ConformalConjugate for Motor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl ConformalConjugate for MotorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl ConformalConjugate for MotorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl ConformalConjugate for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       19        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0() * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5] * -1.0,
            // e41, e42, e43, e45
            self.group3().xyz().with_w(self[e45] * -1.0),
            // e15, e25, e35
            self.group4() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            self.group9() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
            // e3215
            self[e3215] * -1.0,
        )
    }
}
impl ConformalConjugate for MysteryCircle {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        MysteryCircle::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl ConformalConjugate for MysteryCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        MysteryCircleRotor::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e12345
            self[e12345] * -1.0,
        )
    }
}
impl ConformalConjugate for MysteryDipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        MysteryDipole::from_groups(/* e23, e31, e12, e45 */ self.group0().xyz().with_w(self[e45] * -1.0))
    }
}
impl ConformalConjugate for MysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        MysteryDipoleInversion::from_groups(
            // e23, e31, e12, e45
            self.group0().xyz().with_w(self[e45] * -1.0),
            // e4235, e4315, e4125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl ConformalConjugate for MysteryVersorEven {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl ConformalConjugate for MysteryVersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e45] * -1.0),
        )
    }
}
impl ConformalConjugate for NullCircleAtOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl ConformalConjugate for NullDipoleAtOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl ConformalConjugate for NullDipoleInversionAtOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl ConformalConjugate for NullSphereAtOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl ConformalConjugate for NullVersorEvenAtOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl ConformalConjugate for Origin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl ConformalConjugate for Plane {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl ConformalConjugate for PlaneOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conformal_conjugate(self) -> Self {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl ConformalConjugate for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ self.group0(), /* e5 */ self[e5] * -1.0)
    }
}
impl ConformalConjugate for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn conformal_conjugate(self) -> Self {
        RoundPointAtOrigin::from_groups(/* e4, e5 */ self.group0() * Simd32x2::from([1.0, -1.0]))
    }
}
impl ConformalConjugate for Scalar {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl ConformalConjugate for Sphere {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        Sphere::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0() * Simd32x4::from(-1.0), /* e1234 */ self[e1234])
    }
}
impl ConformalConjugate for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn conformal_conjugate(self) -> Self {
        SphereAtOrigin::from_groups(/* e3215, e1234 */ self.group0() * Simd32x2::from([-1.0, 1.0]))
    }
}
impl ConformalConjugate for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        SphereOnOrigin::from_groups(/* e4235, e4315, e4125, e1234 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl ConformalConjugate for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(self[e12345] * -1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from(-1.0),
            // e1, e2, e3, e4
            self.group3(),
        )
    }
}
impl ConformalConjugate for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(self[e12345] * -1.0),
            // e415, e425, e435, e4
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl ConformalConjugate for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn conformal_conjugate(self) -> Self {
        VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl ConformalConjugate for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        VersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ self.group0(), /* e235, e315, e125, e5 */ self.group1() * Simd32x4::from(-1.0))
    }
}
impl ConformalConjugate for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(self[e12345] * -1.0),
            // e415, e425, e435, e4
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl ConformalConjugate for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from(-1.0),
            // e1, e2, e3, e4
            self.group2(),
        )
    }
}
impl ConformalConjugate for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e45] * -1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group3() * Simd32x4::from(-1.0),
        )
    }
}
impl ConformalConjugate for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e45] * -1.0),
            // e4235, e4315, e4125, e3215
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl ConformalConjugate for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            self.group1().xyz().with_w(self[e3215] * -1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
