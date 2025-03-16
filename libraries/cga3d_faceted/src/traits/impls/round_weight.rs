// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 49
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
impl RoundWeight for AntiCircleOnOrigin {
    type Output = NullDipoleAtOrigin;
    fn round_weight(self) -> Self::Output {
        NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0())
    }
}
impl RoundWeight for AntiCircleRotor {
    type Output = NullDipoleAtOrigin;
    fn round_weight(self) -> Self::Output {
        NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0())
    }
}
impl RoundWeight for AntiCircleRotorAligningOrigin {
    type Output = NullDipoleAtOrigin;
    fn round_weight(self) -> Self::Output {
        NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0())
    }
}
impl RoundWeight for AntiCircleRotorOnOrigin {
    type Output = NullDipoleAtOrigin;
    fn round_weight(self) -> Self::Output {
        NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0().xyz())
    }
}
impl RoundWeight for AntiDipoleInversion {
    type Output = NullVersorEvenAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]))
    }
}
impl RoundWeight for AntiDipoleInversionOnOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]))
    }
}
impl RoundWeight for AntiDipoleInversionOrthogonalOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]))
    }
}
impl RoundWeight for AntiDipoleOnOrigin {
    type Output = NullCircleAtOrigin;
    fn round_weight(self) -> Self::Output {
        NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0().xyz())
    }
}
impl RoundWeight for AntiDualNum {
    type Output = NullSphereAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        NullSphereAtOrigin::from_groups(/* e1234 */ self[e1234])
    }
}
impl RoundWeight for AntiSphereOnOrigin {
    type Output = Origin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e4])
    }
}
impl RoundWeight for AntiVersorEvenOnOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]))
    }
}
impl RoundWeight for Circle {
    type Output = NullCircleAtOrigin;
    fn round_weight(self) -> Self::Output {
        NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0())
    }
}
impl RoundWeight for CircleAligningOrigin {
    type Output = NullCircleAtOrigin;
    fn round_weight(self) -> Self::Output {
        NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0())
    }
}
impl RoundWeight for CircleAtOrigin {
    type Output = NullCircleAtOrigin;
    fn round_weight(self) -> Self::Output {
        NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0())
    }
}
impl RoundWeight for CircleOnOrigin {
    type Output = NullCircleAtOrigin;
    fn round_weight(self) -> Self::Output {
        NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0())
    }
}
impl RoundWeight for CircleOrthogonalOrigin {
    type Output = NullCircleAtOrigin;
    fn round_weight(self) -> Self::Output {
        NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0().xyz())
    }
}
impl RoundWeight for CircleRotor {
    type Output = NullCircleAtOrigin;
    fn round_weight(self) -> Self::Output {
        NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0())
    }
}
impl RoundWeight for CircleRotorAligningOrigin {
    type Output = NullCircleAtOrigin;
    fn round_weight(self) -> Self::Output {
        NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0())
    }
}
impl RoundWeight for CircleRotorOnOrigin {
    type Output = NullCircleAtOrigin;
    fn round_weight(self) -> Self::Output {
        NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0().xyz())
    }
}
impl RoundWeight for Dipole {
    type Output = NullDipoleAtOrigin;
    fn round_weight(self) -> Self::Output {
        NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0())
    }
}
impl RoundWeight for DipoleAligningOrigin {
    type Output = NullDipoleAtOrigin;
    fn round_weight(self) -> Self::Output {
        NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0().xyz())
    }
}
impl RoundWeight for DipoleAtOrigin {
    type Output = NullDipoleAtOrigin;
    fn round_weight(self) -> Self::Output {
        NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0())
    }
}
impl RoundWeight for DipoleInversion {
    type Output = NullDipoleInversionAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]))
    }
}
impl RoundWeight for DipoleInversionAligningOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]))
    }
}
impl RoundWeight for DipoleInversionAtOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]))
    }
}
impl RoundWeight for DipoleInversionOnOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]))
    }
}
impl RoundWeight for DipoleInversionOrthogonalOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]))
    }
}
impl RoundWeight for DipoleOnOrigin {
    type Output = NullDipoleAtOrigin;
    fn round_weight(self) -> Self::Output {
        NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0().xyz())
    }
}
impl RoundWeight for DipoleOrthogonalOrigin {
    type Output = NullDipoleAtOrigin;
    fn round_weight(self) -> Self::Output {
        NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0())
    }
}
impl RoundWeight for DualNum {
    type Output = Origin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e4])
    }
}
impl RoundWeight for MultiVector {
    type Output = MultiVector;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group3().xyz().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        )
    }
}
impl RoundWeight for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;
    fn round_weight(self) -> Self::Output {
        self
    }
}
impl RoundWeight for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;
    fn round_weight(self) -> Self::Output {
        self
    }
}
impl RoundWeight for NullDipoleInversionAtOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn round_weight(self) -> Self::Output {
        self
    }
}
impl RoundWeight for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;
    fn round_weight(self) -> Self::Output {
        self
    }
}
impl RoundWeight for NullVersorEvenAtOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn round_weight(self) -> Self::Output {
        self
    }
}
impl RoundWeight for Origin {
    type Output = Origin;
    fn round_weight(self) -> Self::Output {
        self
    }
}
impl RoundWeight for RoundPoint {
    type Output = Origin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e4])
    }
}
impl RoundWeight for RoundPointAtOrigin {
    type Output = Origin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e4])
    }
}
impl RoundWeight for Sphere {
    type Output = NullSphereAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        NullSphereAtOrigin::from_groups(/* e1234 */ self[e1234])
    }
}
impl RoundWeight for SphereAtOrigin {
    type Output = NullSphereAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        NullSphereAtOrigin::from_groups(/* e1234 */ self[e1234])
    }
}
impl RoundWeight for SphereOnOrigin {
    type Output = NullSphereAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        NullSphereAtOrigin::from_groups(/* e1234 */ self[e1234])
    }
}
impl RoundWeight for VersorEven {
    type Output = NullVersorEvenAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]))
    }
}
impl RoundWeight for VersorEvenAligningOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]))
    }
}
impl RoundWeight for VersorEvenAtOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn round_weight(self) -> Self::Output {
        NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ self.group0())
    }
}
impl RoundWeight for VersorEvenOnOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]))
    }
}
impl RoundWeight for VersorEvenOrthogonalOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]))
    }
}
impl RoundWeight for VersorOdd {
    type Output = NullDipoleInversionAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]))
    }
}
impl RoundWeight for VersorOddOrthogonalOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]))
    }
}
