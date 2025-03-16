// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 51
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         0       0       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         0       0       0
impl RoundBulk for AntiCircleOnOrigin {
    type Output = AntiLineOnOrigin;
    fn round_bulk(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group1())
    }
}
impl RoundBulk for AntiCircleRotor {
    type Output = AntiMotorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]))
    }
}
impl RoundBulk for AntiCircleRotorAligningOrigin {
    type Output = AntiMotorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]))
    }
}
impl RoundBulk for AntiCircleRotorAligningOriginAtInfinity {
    type Output = AntiMotorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]))
    }
}
impl RoundBulk for AntiCircleRotorAtInfinity {
    type Output = AntiMotorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]))
    }
}
impl RoundBulk for AntiCircleRotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]))
    }
}
impl RoundBulk for AntiDipoleInversion {
    type Output = AntiFlectorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]))
    }
}
impl RoundBulk for AntiDipoleInversionAtInfinity {
    type Output = AntiFlectorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]))
    }
}
impl RoundBulk for AntiDipoleInversionOnOrigin {
    type Output = AntiFlectorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]))
    }
}
impl RoundBulk for AntiDipoleOnOrigin {
    type Output = AntiFlatOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e321])
    }
}
impl RoundBulk for AntiDualNum {
    type Output = Scalar;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar])
    }
}
impl RoundBulk for AntiFlatOrigin {
    type Output = AntiFlatOrigin;
    fn round_bulk(self) -> Self::Output {
        self
    }
}
impl RoundBulk for AntiFlatPoint {
    type Output = AntiFlatOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e321])
    }
}
impl RoundBulk for AntiFlector {
    type Output = AntiFlectorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]))
    }
}
impl RoundBulk for AntiFlectorOnOrigin {
    type Output = AntiFlectorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        self
    }
}
impl RoundBulk for AntiLine {
    type Output = AntiLineOnOrigin;
    fn round_bulk(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0())
    }
}
impl RoundBulk for AntiLineOnOrigin {
    type Output = AntiLineOnOrigin;
    fn round_bulk(self) -> Self::Output {
        self
    }
}
impl RoundBulk for AntiMotor {
    type Output = AntiMotorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ self.group0())
    }
}
impl RoundBulk for AntiMotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        self
    }
}
impl RoundBulk for AntiMysteryCircleRotor {
    type Output = AntiMotorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]))
    }
}
impl RoundBulk for AntiMysteryDipoleInversion {
    type Output = AntiFlectorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]))
    }
}
impl RoundBulk for AntiPlane {
    type Output = AntiPlaneOnOrigin;
    fn round_bulk(self) -> Self::Output {
        AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0().xyz())
    }
}
impl RoundBulk for AntiPlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn round_bulk(self) -> Self::Output {
        self
    }
}
impl RoundBulk for AntiSphereOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn round_bulk(self) -> Self::Output {
        AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0().xyz())
    }
}
impl RoundBulk for AntiVersorEvenOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]))
    }
}
impl RoundBulk for Circle {
    type Output = AntiFlatOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e321])
    }
}
impl RoundBulk for CircleAtInfinity {
    type Output = AntiFlatOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e321])
    }
}
impl RoundBulk for CircleOrthogonalOrigin {
    type Output = AntiFlatOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e321])
    }
}
impl RoundBulk for CircleRotor {
    type Output = AntiFlatOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e321])
    }
}
impl RoundBulk for CircleRotorAtInfinity {
    type Output = AntiFlatOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e321])
    }
}
impl RoundBulk for Dipole {
    type Output = AntiLineOnOrigin;
    fn round_bulk(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group1().xyz())
    }
}
impl RoundBulk for DipoleAtInfinity {
    type Output = AntiLineOnOrigin;
    fn round_bulk(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0().xyz())
    }
}
impl RoundBulk for DipoleInversion {
    type Output = AntiLineOnOrigin;
    fn round_bulk(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group1().xyz())
    }
}
impl RoundBulk for DipoleInversionAtInfinity {
    type Output = AntiLineOnOrigin;
    fn round_bulk(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0().xyz())
    }
}
impl RoundBulk for DipoleInversionOrthogonalOrigin {
    type Output = AntiLineOnOrigin;
    fn round_bulk(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group1())
    }
}
impl RoundBulk for DipoleOrthogonalOrigin {
    type Output = AntiLineOnOrigin;
    fn round_bulk(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group1())
    }
}
impl RoundBulk for MultiVector {
    type Output = MultiVector;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}
impl RoundBulk for MysteryCircle {
    type Output = AntiFlatOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e321])
    }
}
impl RoundBulk for MysteryCircleRotor {
    type Output = AntiFlatOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e321])
    }
}
impl RoundBulk for MysteryDipole {
    type Output = AntiLineOnOrigin;
    fn round_bulk(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0().xyz())
    }
}
impl RoundBulk for MysteryDipoleInversion {
    type Output = AntiLineOnOrigin;
    fn round_bulk(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0().xyz())
    }
}
impl RoundBulk for MysteryVersorEven {
    type Output = AntiFlectorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]))
    }
}
impl RoundBulk for MysteryVersorOdd {
    type Output = AntiMotorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]))
    }
}
impl RoundBulk for RoundPoint {
    type Output = AntiPlaneOnOrigin;
    fn round_bulk(self) -> Self::Output {
        AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0().xyz())
    }
}
impl RoundBulk for Scalar {
    type Output = Scalar;
    fn round_bulk(self) -> Self::Output {
        self
    }
}
impl RoundBulk for VersorEven {
    type Output = AntiFlectorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]))
    }
}
impl RoundBulk for VersorEvenAtInfinity {
    type Output = AntiFlectorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]))
    }
}
impl RoundBulk for VersorEvenOrthogonalOrigin {
    type Output = AntiFlectorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]))
    }
}
impl RoundBulk for VersorOdd {
    type Output = AntiMotorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]))
    }
}
impl RoundBulk for VersorOddAtInfinity {
    type Output = AntiMotorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]))
    }
}
impl RoundBulk for VersorOddOrthogonalOrigin {
    type Output = AntiMotorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]))
    }
}
