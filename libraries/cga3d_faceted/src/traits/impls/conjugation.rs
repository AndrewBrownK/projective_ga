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
//  Maximum:         0       6       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       3       0
//  Average:         0       3       0
//  Maximum:         0      17       0
impl Conjugation for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn conjugation(self) -> Self {
        AntiCircleOnOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl Conjugation for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn conjugation(self) -> Self {
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl Conjugation for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn conjugation(self) -> Self {
        AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl Conjugation for AntiCircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conjugation(self) -> Self {
        AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl Conjugation for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conjugation(self) -> Self {
        AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl Conjugation for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conjugation(self) -> Self {
        AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl Conjugation for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            self.group2().xyz().with_w(self[e4] * -1.0),
            // e1, e2, e3, e5
            self.group3() * Simd32x4::from(-1.0),
        )
    }
}
impl Conjugation for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl Conjugation for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        AntiDipoleInversionOnOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0(), /* e4, e1, e2, e3 */ self.group1() * Simd32x4::from(-1.0))
    }
}
impl Conjugation for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0().xyz().with_w(self[e5] * -1.0),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            self.group2().xyz().with_w(self[e4] * -1.0),
        )
    }
}
impl Conjugation for AntiDipoleOnOrigin {
    fn conjugation(self) -> Self {
        self
    }
}
impl Conjugation for AntiDualNum {
    fn conjugation(self) -> Self {
        self
    }
}
impl Conjugation for AntiFlatOrigin {
    fn conjugation(self) -> Self {
        self
    }
}
impl Conjugation for AntiFlatPoint {
    fn conjugation(self) -> Self {
        self
    }
}
impl Conjugation for AntiFlector {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        AntiFlector::from_groups(/* e235, e315, e125, e321 */ self.group0(), /* e1, e2, e3, e5 */ self.group1() * Simd32x4::from(-1.0))
    }
}
impl Conjugation for AntiFlectorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]))
    }
}
impl Conjugation for AntiLine {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn conjugation(self) -> Self {
        AntiLine::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl Conjugation for AntiLineOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conjugation(self) -> Self {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl Conjugation for AntiMotor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conjugation(self) -> Self {
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e3215
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl Conjugation for AntiMotorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl Conjugation for AntiMysteryCircleRotor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        AntiMysteryCircleRotor::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0), /* scalar */ self[scalar])
    }
}
impl Conjugation for AntiMysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conjugation(self) -> Self {
        AntiMysteryDipoleInversion::from_groups(/* e415, e425, e435, e321 */ self.group0(), /* e1, e2, e3 */ self.group1() * Simd32x3::from(-1.0))
    }
}
impl Conjugation for AntiPlane {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl Conjugation for AntiPlaneOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conjugation(self) -> Self {
        AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl Conjugation for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e12345] * -1.0)
    }
}
impl Conjugation for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        AntiSphereOnOrigin::from_groups(/* e1, e2, e3, e4 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl Conjugation for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conjugation(self) -> Self {
        AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl Conjugation for Circle {
    fn conjugation(self) -> Self {
        self
    }
}
impl Conjugation for CircleAligningOrigin {
    fn conjugation(self) -> Self {
        self
    }
}
impl Conjugation for CircleAtInfinity {
    fn conjugation(self) -> Self {
        self
    }
}
impl Conjugation for CircleAtOrigin {
    fn conjugation(self) -> Self {
        self
    }
}
impl Conjugation for CircleOnOrigin {
    fn conjugation(self) -> Self {
        self
    }
}
impl Conjugation for CircleOrthogonalOrigin {
    fn conjugation(self) -> Self {
        self
    }
}
impl Conjugation for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e12345
            self.group2().xyz().with_w(self[e12345] * -1.0),
        )
    }
}
impl Conjugation for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e12345
            self.group2().xyz().with_w(self[e12345] * -1.0),
        )
    }
}
impl Conjugation for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            self.group0(),
            // e235, e315, e125, e12345
            self.group1().xyz().with_w(self[e12345] * -1.0),
        )
    }
}
impl Conjugation for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        CircleRotorAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e12345
            self.group1().xyz().with_w(self[e12345] * -1.0),
        )
    }
}
impl Conjugation for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(self[e12345] * -1.0),
            // e415, e425, e435
            self.group1(),
        )
    }
}
impl Conjugation for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn conjugation(self) -> Self {
        Dipole::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group2() * Simd32x3::from(-1.0),
        )
    }
}
impl Conjugation for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conjugation(self) -> Self {
        DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl Conjugation for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conjugation(self) -> Self {
        DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl Conjugation for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn conjugation(self) -> Self {
        DipoleAtOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl Conjugation for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn conjugation(self) -> Self {
        DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        )
    }
}
impl Conjugation for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conjugation(self) -> Self {
        DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        )
    }
}
impl Conjugation for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conjugation(self) -> Self {
        DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group2(),
        )
    }
}
impl Conjugation for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conjugation(self) -> Self {
        DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl Conjugation for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        DipoleInversionOnOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0() * Simd32x4::from(-1.0), /* e1234, e4235, e4315, e4125 */ self.group1())
    }
}
impl Conjugation for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn conjugation(self) -> Self {
        DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl Conjugation for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl Conjugation for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn conjugation(self) -> Self {
        DipoleOrthogonalOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group2() * Simd32x3::from(-1.0),
        )
    }
}
impl Conjugation for DualNum {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn conjugation(self) -> Self {
        DualNum::from_groups(/* e4, e12345 */ self.group0() * Simd32x2::from(-1.0))
    }
}
impl Conjugation for FlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ self[e45] * -1.0)
    }
}
impl Conjugation for FlatPoint {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl Conjugation for FlatPointAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conjugation(self) -> Self {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl Conjugation for Flector {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        Flector::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0), /* e4235, e4315, e4125, e3215 */ self.group1())
    }
}
impl Conjugation for FlectorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl Conjugation for FlectorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]))
    }
}
impl Conjugation for Horizon {
    fn conjugation(self) -> Self {
        self
    }
}
impl Conjugation for Infinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ self[e5] * -1.0)
    }
}
impl Conjugation for Line {
    fn conjugation(self) -> Self {
        self
    }
}
impl Conjugation for LineAtInfinity {
    fn conjugation(self) -> Self {
        self
    }
}
impl Conjugation for LineOnOrigin {
    fn conjugation(self) -> Self {
        self
    }
}
impl Conjugation for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().xyz().with_w(self[e12345] * -1.0),
            // e235, e315, e125, e5
            self.group1().xyz().with_w(self[e5] * -1.0),
        )
    }
}
impl Conjugation for MotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0().xyz().with_w(self[e5] * -1.0))
    }
}
impl Conjugation for MotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0().xyz().with_w(self[e12345] * -1.0))
    }
}
impl Conjugation for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       17        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0() * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            self.group1() * Simd32x4::from(-1.0),
            // e5
            self[e5] * -1.0,
            // e41, e42, e43, e45
            self.group3() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group4() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group5() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        )
    }
}
impl Conjugation for MysteryCircle {
    fn conjugation(self) -> Self {
        self
    }
}
impl Conjugation for MysteryCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        MysteryCircleRotor::from_groups(/* e415, e425, e435, e321 */ self.group0(), /* e12345 */ self[e12345] * -1.0)
    }
}
impl Conjugation for MysteryDipole {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        MysteryDipole::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl Conjugation for MysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        MysteryDipoleInversion::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0), /* e4235, e4315, e4125 */ self.group1())
    }
}
impl Conjugation for MysteryVersorEven {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        MysteryVersorEven::from_groups(/* e12345, e1, e2, e3 */ self.group0() * Simd32x4::from(-1.0), /* e415, e425, e435, e321 */ self.group1())
    }
}
impl Conjugation for MysteryVersorOdd {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        MysteryVersorOdd::from_groups(/* scalar, e4235, e4315, e4125 */ self.group0(), /* e23, e31, e12, e45 */ self.group1() * Simd32x4::from(-1.0))
    }
}
impl Conjugation for NullCircleAtOrigin {
    fn conjugation(self) -> Self {
        self
    }
}
impl Conjugation for NullDipoleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conjugation(self) -> Self {
        NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl Conjugation for NullDipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl Conjugation for NullSphereAtOrigin {
    fn conjugation(self) -> Self {
        self
    }
}
impl Conjugation for NullVersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ self.group0().xyz().with_w(self[e4] * -1.0))
    }
}
impl Conjugation for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e4] * -1.0)
    }
}
impl Conjugation for Plane {
    fn conjugation(self) -> Self {
        self
    }
}
impl Conjugation for PlaneOnOrigin {
    fn conjugation(self) -> Self {
        self
    }
}
impl Conjugation for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ self.group0() * Simd32x4::from(-1.0), /* e5 */ self[e5] * -1.0)
    }
}
impl Conjugation for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn conjugation(self) -> Self {
        RoundPointAtOrigin::from_groups(/* e4, e5 */ self.group0() * Simd32x2::from(-1.0))
    }
}
impl Conjugation for Scalar {
    fn conjugation(self) -> Self {
        self
    }
}
impl Conjugation for Sphere {
    fn conjugation(self) -> Self {
        self
    }
}
impl Conjugation for SphereAtOrigin {
    fn conjugation(self) -> Self {
        self
    }
}
impl Conjugation for SphereOnOrigin {
    fn conjugation(self) -> Self {
        self
    }
}
impl Conjugation for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(self[e12345] * -1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2().xyz().with_w(self[e5] * -1.0),
            // e1, e2, e3, e4
            self.group3() * Simd32x4::from(-1.0),
        )
    }
}
impl Conjugation for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(self[e12345] * -1.0),
            // e415, e425, e435, e4
            self.group1().xyz().with_w(self[e4] * -1.0),
            // e235, e315, e125, e5
            self.group2().xyz().with_w(self[e5] * -1.0),
        )
    }
}
impl Conjugation for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2().xyz().with_w(self[e5] * -1.0),
        )
    }
}
impl Conjugation for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            self.group0().xyz().with_w(self[e4] * -1.0),
            // e235, e315, e125, e5
            self.group1().xyz().with_w(self[e5] * -1.0),
        )
    }
}
impl Conjugation for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(self[e12345] * -1.0),
            // e415, e425, e435, e4
            self.group1().xyz().with_w(self[e4] * -1.0),
        )
    }
}
impl Conjugation for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1().xyz().with_w(self[e5] * -1.0),
            // e1, e2, e3, e4
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl Conjugation for VersorOdd {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn conjugation(self) -> Self {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        )
    }
}
impl Conjugation for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conjugation(self) -> Self {
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group2(),
        )
    }
}
impl Conjugation for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn conjugation(self) -> Self {
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, e3215
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
