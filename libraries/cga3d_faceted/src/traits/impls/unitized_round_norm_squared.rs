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
//  Average:         0       0       0
//  Maximum:         2      13       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         2      21       0
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiCircleOnOrigin {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e41], 2) * f32::powi(self[e23], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiCircleRotor {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e41], 2) * f32::powi(self[e23], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiCircleRotorAligningOrigin {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self.group1().with_w(self[scalar])[0], 2) * f32::powi(self[e41], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiCircleRotorOnOrigin {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self.group0().xyz()[0], 2) * f32::powi(self.group1().with_w(self[scalar])[0], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiDipoleInversion {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self.group0().with_w(self[e4]).wxyz()[0], 2) * f32::powi(self[e321], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiDipoleInversionOnOrigin {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e321], 2) * f32::powi(self[e4], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiFlatOrigin::from_groups(/* e321 */ self[e321]);
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz());
        return (f32::powi(sub_type[e321], 2) * f32::powi(wedge[e4235], 2))
            + (f32::powi(sub_type[e321], 2) * f32::powi(wedge[e4315], 2))
            + (f32::powi(sub_type[e321], 2) * f32::powi(wedge[e4125], 2));
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for AntiDualNum {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiDualNum {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[scalar], 2) * f32::powi(self[e1234], -2);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0().xyz());
        let wedge = FlatOrigin::from_groups(/* e45 */ self[e4]);
        return (f32::powi(sub_type[e1], 2) * f32::powi(wedge[e45], -2))
            + (f32::powi(sub_type[e2], 2) * f32::powi(wedge[e45], -2))
            + (f32::powi(sub_type[e3], 2) * f32::powi(wedge[e45], -2));
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiVersorEvenOnOrigin {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e41], 2) * f32::powi(self[e23], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for Circle {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiFlatOrigin::from_groups(/* e321 */ self[e321]);
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
        return (f32::powi(sub_type[e321], 2) * f32::powi(wedge[e4235], 2))
            + (f32::powi(sub_type[e321], 2) * f32::powi(wedge[e4315], 2))
            + (f32::powi(sub_type[e321], 2) * f32::powi(wedge[e4125], 2));
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiFlatOrigin::from_groups(/* e321 */ self[e321]);
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz());
        return (f32::powi(sub_type[e321], 2) * f32::powi(wedge[e4235], 2))
            + (f32::powi(sub_type[e321], 2) * f32::powi(wedge[e4315], 2))
            + (f32::powi(sub_type[e321], 2) * f32::powi(wedge[e4125], 2));
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiFlatOrigin::from_groups(/* e321 */ self[e321]);
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
        return (f32::powi(sub_type[e321], 2) * f32::powi(wedge[e4235], 2))
            + (f32::powi(sub_type[e321], 2) * f32::powi(wedge[e4315], 2))
            + (f32::powi(sub_type[e321], 2) * f32::powi(wedge[e4125], 2));
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for Dipole {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for Dipole {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self.group1().xyz()[0], 2) * f32::powi(self[e41], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for DipoleInversion {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self.group1().xyz()[0], 2) * f32::powi(self.group0().with_w(self[e1234])[0], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for DipoleInversionOrthogonalOrigin {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e41], 2) * f32::powi(self[e23], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for DipoleOrthogonalOrigin {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e41], 2) * f32::powi(self[e23], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        9        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0       13        0
    //  no simd        0       21        0
    fn unitized_round_norm_squared(self) -> f32 {
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
        return sub_type[e41] * sub_type[e15] * wedge[e423] * wedge[e235] * 4.0;
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for RoundPoint {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0().xyz());
        let wedge = FlatOrigin::from_groups(/* e45 */ self[e4]);
        return (f32::powi(sub_type[e1], 2) * f32::powi(wedge[e45], -2))
            + (f32::powi(sub_type[e2], 2) * f32::powi(wedge[e45], -2))
            + (f32::powi(sub_type[e3], 2) * f32::powi(wedge[e45], -2));
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for VersorEven {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e321], 2) * f32::powi(self[e4], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for VersorEvenOrthogonalOrigin {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e321], 2) * f32::powi(self[e4], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for VersorOdd {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e41], 2) * f32::powi(self[e23], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for VersorOddOrthogonalOrigin {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e41], 2) * f32::powi(self[e23], 2);
    }
}
