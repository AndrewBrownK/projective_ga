// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 95
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         0       1       0     N/A
//  Average:         0       0       0     N/A
//  Maximum:         0       6       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         0       1       0       0
//  Average:         0       3       0       0
//  Maximum:         0      17       0       0
impl AutoMorphism for AntiCircleOnOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for AntiCircleRotor {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for AntiCircleRotorAligningOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for AntiCircleRotorAligningOriginAtInfinity {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for AntiCircleRotorAtInfinity {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for AntiCircleRotorOnOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       15        0        0
    fn auto_morphism(self) -> Self {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from(-1.0),
            // e1, e2, e3, e5
            self.group3() * Simd32x4::from(-1.0),
        )
    }
}
impl AutoMorphism for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       11        0        0
    fn auto_morphism(self) -> Self {
        AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
            // e1, e2, e3, e5
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl AutoMorphism for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn auto_morphism(self) -> Self {
        AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from(-1.0),
            // e4, e1, e2, e3
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl AutoMorphism for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       11        0        0
    fn auto_morphism(self) -> Self {
        AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl AutoMorphism for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn auto_morphism(self) -> Self {
        AntiDipoleOnOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl AutoMorphism for AntiDualNum {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for AntiFlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn auto_morphism(self) -> Self {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e321] * -1.0)
    }
}
impl AutoMorphism for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn auto_morphism(self) -> Self {
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl AutoMorphism for AntiFlector {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn auto_morphism(self) -> Self {
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group0() * Simd32x4::from(-1.0),
            // e1, e2, e3, e5
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl AutoMorphism for AntiFlectorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn auto_morphism(self) -> Self {
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl AutoMorphism for AntiLine {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for AntiLineOnOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for AntiMotor {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for AntiMotorOnOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for AntiMysteryCircleRotor {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for AntiMysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        7        0        0
    fn auto_morphism(self) -> Self {
        AntiMysteryDipoleInversion::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from(-1.0),
            // e1, e2, e3
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl AutoMorphism for AntiPlane {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn auto_morphism(self) -> Self {
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl AutoMorphism for AntiPlaneOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn auto_morphism(self) -> Self {
        AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl AutoMorphism for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn auto_morphism(self) -> Self {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e12345] * -1.0)
    }
}
impl AutoMorphism for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn auto_morphism(self) -> Self {
        AntiSphereOnOrigin::from_groups(/* e1, e2, e3, e4 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl AutoMorphism for AntiVersorEvenOnOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       10        0        0
    fn auto_morphism(self) -> Self {
        Circle::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group2() * Simd32x3::from(-1.0),
        )
    }
}
impl AutoMorphism for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        3        0      N/A
    // no simd        0        9        0        0
    fn auto_morphism(self) -> Self {
        CircleAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group2() * Simd32x3::from(-1.0),
        )
    }
}
impl AutoMorphism for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        7        0        0
    fn auto_morphism(self) -> Self {
        CircleAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl AutoMorphism for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn auto_morphism(self) -> Self {
        CircleAtOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl AutoMorphism for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn auto_morphism(self) -> Self {
        CircleOnOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl AutoMorphism for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        7        0        0
    fn auto_morphism(self) -> Self {
        CircleOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl AutoMorphism for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       11        0        0
    fn auto_morphism(self) -> Self {
        CircleRotor::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl AutoMorphism for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       10        0        0
    fn auto_morphism(self) -> Self {
        CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl AutoMorphism for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        7        0        0
    fn auto_morphism(self) -> Self {
        CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl AutoMorphism for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn auto_morphism(self) -> Self {
        CircleRotorAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl AutoMorphism for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        7        0        0
    fn auto_morphism(self) -> Self {
        CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl AutoMorphism for Dipole {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for DipoleAligningOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for DipoleAtInfinity {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for DipoleAtOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for DipoleInversion {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for DipoleInversionAligningOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for DipoleInversionAtInfinity {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for DipoleInversionAtOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for DipoleInversionOnOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for DipoleInversionOrthogonalOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for DipoleOnOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for DipoleOrthogonalOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for DualNum {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd2        0        1        0      N/A
    // no simd        0        2        0        0
    fn auto_morphism(self) -> Self {
        DualNum::from_groups(/* e4, e12345 */ self.group0() * Simd32x2::from(-1.0))
    }
}
impl AutoMorphism for FlatOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for FlatPoint {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for FlatPointAtInfinity {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for Flector {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for FlectorAtInfinity {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for FlectorOnOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for Horizon {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for Infinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn auto_morphism(self) -> Self {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ self[e5] * -1.0)
    }
}
impl AutoMorphism for Line {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn auto_morphism(self) -> Self {
        Line::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl AutoMorphism for LineAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn auto_morphism(self) -> Self {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl AutoMorphism for LineOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn auto_morphism(self) -> Self {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl AutoMorphism for Motor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn auto_morphism(self) -> Self {
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl AutoMorphism for MotorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn auto_morphism(self) -> Self {
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl AutoMorphism for MotorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn auto_morphism(self) -> Self {
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl AutoMorphism for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       17        0        0
    fn auto_morphism(self) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0() * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            self.group1() * Simd32x4::from(-1.0),
            // e5
            self[e5] * -1.0,
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() * Simd32x4::from(-1.0),
            // e423, e431, e412
            self.group7() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group8() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        )
    }
}
impl AutoMorphism for MysteryCircle {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn auto_morphism(self) -> Self {
        MysteryCircle::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl AutoMorphism for MysteryCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn auto_morphism(self) -> Self {
        use crate::elements::*;
        MysteryCircleRotor::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from(-1.0), /* e12345 */ self[e12345] * -1.0)
    }
}
impl AutoMorphism for MysteryDipole {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for MysteryDipoleInversion {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for MysteryVersorEven {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn auto_morphism(self) -> Self {
        MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            self.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl AutoMorphism for MysteryVersorOdd {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for NullCircleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn auto_morphism(self) -> Self {
        NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl AutoMorphism for NullDipoleAtOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for NullDipoleInversionAtOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for NullSphereAtOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for NullVersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn auto_morphism(self) -> Self {
        NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl AutoMorphism for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn auto_morphism(self) -> Self {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e4] * -1.0)
    }
}
impl AutoMorphism for Plane {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for PlaneOnOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn auto_morphism(self) -> Self {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ self.group0() * Simd32x4::from(-1.0), /* e5 */ self[e5] * -1.0)
    }
}
impl AutoMorphism for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd2        0        1        0      N/A
    // no simd        0        2        0        0
    fn auto_morphism(self) -> Self {
        RoundPointAtOrigin::from_groups(/* e4, e5 */ self.group0() * Simd32x2::from(-1.0))
    }
}
impl AutoMorphism for Scalar {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for Sphere {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for SphereAtOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for SphereOnOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for VersorEven {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
    fn auto_morphism(self) -> Self {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from(-1.0),
            // e1, e2, e3, e4
            self.group3() * Simd32x4::from(-1.0),
        )
    }
}
impl AutoMorphism for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        3        0      N/A
    // no simd        0       12        0        0
    fn auto_morphism(self) -> Self {
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435, e4
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl AutoMorphism for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        3        0      N/A
    // no simd        0       12        0        0
    fn auto_morphism(self) -> Self {
        VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl AutoMorphism for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn auto_morphism(self) -> Self {
        VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl AutoMorphism for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn auto_morphism(self) -> Self {
        VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435, e4
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl AutoMorphism for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        3        0      N/A
    // no simd        0       12        0        0
    fn auto_morphism(self) -> Self {
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from(-1.0),
            // e1, e2, e3, e4
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl AutoMorphism for VersorOdd {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for VersorOddAtInfinity {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl AutoMorphism for VersorOddOrthogonalOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
