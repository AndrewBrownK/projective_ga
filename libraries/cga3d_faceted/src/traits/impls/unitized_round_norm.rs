// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 23
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       1       0
//  Maximum:         2      13       3
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       1       0
//  Maximum:         2      21       3
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiCircleOnOrigin {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e41], 2) * f32::powi(self[e23], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiCircleRotor {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e41], 2) * f32::powi(self[e23], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiCircleRotorAligningOrigin {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e41], 2) * f32::powi(self[e23], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiCircleRotorOnOrigin {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e41], 2) * f32::powi(self[e23], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiDipoleInversion {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e321], 2) * f32::powi(self[e4], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiDipoleInversionOnOrigin {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e321], 2) * f32::powi(self[e4], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiFlatOrigin::from_groups(/* e321 */ self[e321]);
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz());
        return (f32::powi(wedge[e4235], 2) * sub_type[e321]) + (f32::powi(wedge[e4315], 2) * sub_type[e321]) + (f32::powi(wedge[e4125], 2) * sub_type[e321]);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiDualNum {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        return self[scalar] / (self[e1234]);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        3
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0().xyz());
        let wedge = FlatOrigin::from_groups(/* e45 */ self[e4]);
        return (f32::powi(sub_type[e1], 2) / (wedge[e45])) + (f32::powi(sub_type[e2], 2) / (wedge[e45])) + (f32::powi(sub_type[e3], 2) / (wedge[e45]));
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiVersorEvenOnOrigin {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e41], 2) * f32::powi(self[e23], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for Circle {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiFlatOrigin::from_groups(/* e321 */ self[e321]);
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
        return (f32::powi(wedge[e4235], 2) * sub_type[e321]) + (f32::powi(wedge[e4315], 2) * sub_type[e321]) + (f32::powi(wedge[e4125], 2) * sub_type[e321]);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiFlatOrigin::from_groups(/* e321 */ self[e321]);
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz());
        return (f32::powi(wedge[e4235], 2) * sub_type[e321]) + (f32::powi(wedge[e4315], 2) * sub_type[e321]) + (f32::powi(wedge[e4125], 2) * sub_type[e321]);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiFlatOrigin::from_groups(/* e321 */ self[e321]);
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
        return (f32::powi(wedge[e4235], 2) * sub_type[e321]) + (f32::powi(wedge[e4315], 2) * sub_type[e321]) + (f32::powi(wedge[e4125], 2) * sub_type[e321]);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for Dipole {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for Dipole {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e41], 2) * f32::powi(self[e23], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for DipoleInversion {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e41], 2) * f32::powi(self[e23], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for DipoleInversionOrthogonalOrigin {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e41], 2) * f32::powi(self[e23], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for DipoleOrthogonalOrigin {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e41], 2) * f32::powi(self[e23], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        9        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0       13        0
    //  no simd        0       21        0
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        let sub_type = MultiVector::from_groups(
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
        );
        let sub_type_2 = MultiVector::from_groups(
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
        );
        let other = Infinity::from_groups(/* e5 */ 1.0);
        let wedge = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e5] * sub_type_2[e1234]]) * Simd32x2::from([0.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e5] * sub_type_2[e4]),
            // e15, e25, e35
            Simd32x3::from(other[e5]) * sub_type_2.group1().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (Simd32x3::from(other[e5]) * sub_type_2.group3().xyz()).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e5] * sub_type_2[e423], other[e5] * sub_type_2[e431], other[e5] * sub_type_2[e412]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]),
            // e3215
            0.0,
        );
        return (sub_type[e41] * sub_type[e15] * wedge[e4] * wedge[e5]) * 4.0;
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for RoundPoint {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        3
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0().xyz());
        let wedge = FlatOrigin::from_groups(/* e45 */ self[e4]);
        return (f32::powi(sub_type[e1], 2) / (wedge[e45])) + (f32::powi(sub_type[e2], 2) / (wedge[e45])) + (f32::powi(sub_type[e3], 2) / (wedge[e45]));
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for VersorEven {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e321], 2) * f32::powi(self[e4], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for VersorEvenOrthogonalOrigin {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e321], 2) * f32::powi(self[e4], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for VersorOdd {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e41], 2) * f32::powi(self[e23], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for VersorOddOrthogonalOrigin {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e41], 2) * f32::powi(self[e23], 2);
    }
}
