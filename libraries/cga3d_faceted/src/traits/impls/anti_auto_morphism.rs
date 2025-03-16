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
//  Average:         0       0       0
//  Maximum:         0       6       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         0       3       0
//  Maximum:         0      17       0
impl AntiAutoMorphism for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_auto_morphism(self) -> Self {
        AntiCircleOnOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl AntiAutoMorphism for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn anti_auto_morphism(self) -> Self {
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl AntiAutoMorphism for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn anti_auto_morphism(self) -> Self {
        AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl AntiAutoMorphism for AntiCircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_auto_morphism(self) -> Self {
        AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl AntiAutoMorphism for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_auto_morphism(self) -> Self {
        AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl AntiAutoMorphism for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_auto_morphism(self) -> Self {
        AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl AntiAutoMorphism for AntiDipoleInversion {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for AntiDipoleInversionAtInfinity {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for AntiDipoleInversionOnOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for AntiDipoleInversionOrthogonalOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for AntiDipoleOnOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for AntiDualNum {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn anti_auto_morphism(self) -> Self {
        AntiDualNum::from_groups(/* e1234, scalar */ self.group0() * Simd32x2::from(-1.0))
    }
}
impl AntiAutoMorphism for AntiFlatOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for AntiFlatPoint {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for AntiFlector {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for AntiFlectorOnOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for AntiLine {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_auto_morphism(self) -> Self {
        AntiLine::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl AntiAutoMorphism for AntiLineOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_auto_morphism(self) -> Self {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl AntiAutoMorphism for AntiMotor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_auto_morphism(self) -> Self {
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35, e3215
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl AntiAutoMorphism for AntiMotorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_auto_morphism(self) -> Self {
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl AntiAutoMorphism for AntiMysteryCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_auto_morphism(self) -> Self {
        use crate::elements::*;
        AntiMysteryCircleRotor::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0), /* scalar */ self[scalar] * -1.0)
    }
}
impl AntiAutoMorphism for AntiMysteryDipoleInversion {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for AntiPlane {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for AntiPlaneOnOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for AntiScalar {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for AntiSphereOnOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_auto_morphism(self) -> Self {
        AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12, e1234
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl AntiAutoMorphism for Circle {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for CircleAligningOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for CircleAtInfinity {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for CircleAtOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for CircleOnOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for CircleOrthogonalOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for CircleRotor {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for CircleRotorAligningOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for CircleRotorAligningOriginAtInfinity {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for CircleRotorAtInfinity {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for CircleRotorOnOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn anti_auto_morphism(self) -> Self {
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
impl AntiAutoMorphism for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_auto_morphism(self) -> Self {
        DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl AntiAutoMorphism for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_auto_morphism(self) -> Self {
        DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl AntiAutoMorphism for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_auto_morphism(self) -> Self {
        DipoleAtOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl AntiAutoMorphism for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn anti_auto_morphism(self) -> Self {
        DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group3() * Simd32x4::from(-1.0),
        )
    }
}
impl AntiAutoMorphism for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn anti_auto_morphism(self) -> Self {
        DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group1() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl AntiAutoMorphism for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn anti_auto_morphism(self) -> Self {
        DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl AntiAutoMorphism for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_auto_morphism(self) -> Self {
        DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl AntiAutoMorphism for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_auto_morphism(self) -> Self {
        DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from(-1.0),
            // e1234, e4235, e4315, e4125
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl AntiAutoMorphism for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn anti_auto_morphism(self) -> Self {
        DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl AntiAutoMorphism for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_auto_morphism(self) -> Self {
        DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl AntiAutoMorphism for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn anti_auto_morphism(self) -> Self {
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
impl AntiAutoMorphism for DualNum {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for FlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_auto_morphism(self) -> Self {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ self[e45] * -1.0)
    }
}
impl AntiAutoMorphism for FlatPoint {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_auto_morphism(self) -> Self {
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl AntiAutoMorphism for FlatPointAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_auto_morphism(self) -> Self {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl AntiAutoMorphism for Flector {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_auto_morphism(self) -> Self {
        Flector::from_groups(
            // e15, e25, e35, e45
            self.group0() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl AntiAutoMorphism for FlectorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_auto_morphism(self) -> Self {
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl AntiAutoMorphism for FlectorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_auto_morphism(self) -> Self {
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl AntiAutoMorphism for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_auto_morphism(self) -> Self {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[e3215] * -1.0)
    }
}
impl AntiAutoMorphism for Infinity {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for Line {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for LineAtInfinity {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for LineOnOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for Motor {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for MotorAtInfinity {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for MotorOnOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       17        0
    fn anti_auto_morphism(self) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
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
            self.group9() * Simd32x4::from(-1.0),
            // e3215
            self[e3215] * -1.0,
        )
    }
}
impl AntiAutoMorphism for MysteryCircle {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for MysteryCircleRotor {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for MysteryDipole {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_auto_morphism(self) -> Self {
        MysteryDipole::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl AntiAutoMorphism for MysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_auto_morphism(self) -> Self {
        MysteryDipoleInversion::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl AntiAutoMorphism for MysteryVersorEven {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for MysteryVersorOdd {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_auto_morphism(self) -> Self {
        MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            self.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl AntiAutoMorphism for NullCircleAtOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for NullDipoleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_auto_morphism(self) -> Self {
        NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl AntiAutoMorphism for NullDipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_auto_morphism(self) -> Self {
        NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl AntiAutoMorphism for NullSphereAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_auto_morphism(self) -> Self {
        use crate::elements::*;
        NullSphereAtOrigin::from_groups(/* e1234 */ self[e1234] * -1.0)
    }
}
impl AntiAutoMorphism for NullVersorEvenAtOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for Origin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for Plane {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_auto_morphism(self) -> Self {
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl AntiAutoMorphism for PlaneOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_auto_morphism(self) -> Self {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl AntiAutoMorphism for RoundPoint {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for RoundPointAtOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_auto_morphism(self) -> Self {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * -1.0)
    }
}
impl AntiAutoMorphism for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_auto_morphism(self) -> Self {
        use crate::elements::*;
        Sphere::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0() * Simd32x4::from(-1.0), /* e1234 */ self[e1234] * -1.0)
    }
}
impl AntiAutoMorphism for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn anti_auto_morphism(self) -> Self {
        SphereAtOrigin::from_groups(/* e3215, e1234 */ self.group0() * Simd32x2::from(-1.0))
    }
}
impl AntiAutoMorphism for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_auto_morphism(self) -> Self {
        SphereOnOrigin::from_groups(/* e4235, e4315, e4125, e1234 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl AntiAutoMorphism for VersorEven {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for VersorEvenAligningOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for VersorEvenAtInfinity {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for VersorEvenAtOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for VersorEvenOnOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for VersorEvenOrthogonalOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl AntiAutoMorphism for VersorOdd {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn anti_auto_morphism(self) -> Self {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group3() * Simd32x4::from(-1.0),
        )
    }
}
impl AntiAutoMorphism for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn anti_auto_morphism(self) -> Self {
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl AntiAutoMorphism for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn anti_auto_morphism(self) -> Self {
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12, e3215
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
