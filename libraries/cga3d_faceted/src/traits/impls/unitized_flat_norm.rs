// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 30
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         2      13       3
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       1       0
//  Maximum:         2      19       3
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        3
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group2().xyz());
        let sub_type_2 = FlatOrigin::from_groups(/* e45 */ self[e45]);
        return -(f32::powi(wedge[e415], 2) / (sub_type_2[e45])) - (f32::powi(wedge[e425], 2) / (sub_type_2[e45])) - (f32::powi(wedge[e435], 2) / (sub_type_2[e45]));
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        3
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1().xyz());
        let sub_type_2 = FlatOrigin::from_groups(/* e45 */ self[e45]);
        return -(f32::powi(wedge[e415], 2) / (sub_type_2[e45])) - (f32::powi(wedge[e425], 2) / (sub_type_2[e45])) - (f32::powi(wedge[e435], 2) / (sub_type_2[e45]));
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for AntiDipoleInversion {
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e415], 2) * f32::powi(self[e5], 2);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for AntiDipoleInversionAtInfinity {
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e415], 2) * f32::powi(self[e5], 2);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for AntiDipoleInversionOrthogonalOrigin {
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e5], 2) * f32::powi(self[e415], 2);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for Circle {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for Circle {
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e415], 2) * f32::powi(self[e235], 2);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for CircleAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for CircleAligningOrigin {
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e415], 2) * f32::powi(self[e235], 2);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for CircleAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for CircleAtInfinity {
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e415], 2) * f32::powi(self[e235], 2);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for CircleRotor {
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e415], 2) * f32::powi(self[e235], 2);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for CircleRotorAligningOrigin {
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e415], 2) * f32::powi(self[e235], 2);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for CircleRotorAligningOriginAtInfinity {
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e415], 2) * f32::powi(self[e235], 2);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for CircleRotorAtInfinity {
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e415], 2) * f32::powi(self[e235], 2);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for Dipole {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        3
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group2());
        let sub_type_2 = FlatOrigin::from_groups(/* e45 */ self[e45]);
        return -(f32::powi(wedge[e415], 2) / (sub_type_2[e45])) - (f32::powi(wedge[e425], 2) / (sub_type_2[e45])) - (f32::powi(wedge[e435], 2) / (sub_type_2[e45]));
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        3
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1());
        let sub_type_2 = FlatOrigin::from_groups(/* e45 */ self[e45]);
        return -(f32::powi(wedge[e415], 2) / (sub_type_2[e45])) - (f32::powi(wedge[e425], 2) / (sub_type_2[e45])) - (f32::powi(wedge[e435], 2) / (sub_type_2[e45]));
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for DipoleAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        3
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1());
        let sub_type_2 = FlatOrigin::from_groups(/* e45 */ self[e45]);
        return -(f32::powi(wedge[e415], 2) / (sub_type_2[e45])) - (f32::powi(wedge[e425], 2) / (sub_type_2[e45])) - (f32::powi(wedge[e435], 2) / (sub_type_2[e45]));
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e45], 2) * f32::powi(self[e15], 2) * -1.0;
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e45], 2) * f32::powi(self[e15], 2) * -1.0;
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e45], 2) * f32::powi(self[e15], 2) * -1.0;
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for FlatPoint {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        3
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz());
        let sub_type_2 = FlatOrigin::from_groups(/* e45 */ self[e45]);
        return -(f32::powi(wedge[e415], 2) / (sub_type_2[e45])) - (f32::powi(wedge[e425], 2) / (sub_type_2[e45])) - (f32::powi(wedge[e435], 2) / (sub_type_2[e45]));
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for Flector {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e15], 2) * f32::powi(self[e45], 2) * -1.0;
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for Line {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for Line {
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e415], 2) * f32::powi(self[e235], 2);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for Motor {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for Motor {
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e415], 2) * f32::powi(self[e5], 2);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       10        0
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0       13        0
    //  no simd        0       19        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        let sub_type = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            self[e3215],
        );
        let other = Origin::from_groups(/* e4 */ 1.0);
        let wedge = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, sub_type[e3215] * other[e4]]) * Simd32x2::from([0.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(sub_type[e5] * other[e4] * -1.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (Simd32x3::from(other[e4]) * sub_type.group4()).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, sub_type[e235] * other[e4], sub_type[e315] * other[e4], sub_type[e125] * other[e4]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            0.0,
        );
        let sub_type_2 = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(self[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group6().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            0.0,
        );
        return (sub_type_2[e4] * sub_type_2[e5] * wedge[e41] * wedge[e15]) * 4.0;
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for Plane {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        let wedge = AntiScalar::from_groups(/* e12345 */ self[e3215]);
        let sub_type_2 = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz());
        return -(f32::powi(sub_type_2[e4235], 2) * wedge[e12345]) - (f32::powi(sub_type_2[e4315], 2) * wedge[e12345]) - (f32::powi(sub_type_2[e4125], 2) * wedge[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for Sphere {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        let wedge = AntiScalar::from_groups(/* e12345 */ self[e3215]);
        let sub_type_2 = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz());
        return -(f32::powi(sub_type_2[e4235], 2) * wedge[e12345]) - (f32::powi(sub_type_2[e4315], 2) * wedge[e12345]) - (f32::powi(sub_type_2[e4125], 2) * wedge[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for VersorEven {
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e415], 2) * f32::powi(self[e5], 2);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for VersorEvenAligningOrigin {
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e415], 2) * f32::powi(self[e5], 2);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for VersorEvenAtInfinity {
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e415], 2) * f32::powi(self[e5], 2);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e45], 2) * f32::powi(self[e15], 2) * -1.0;
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e15], 2) * f32::powi(self[e45], 2) * -1.0;
    }
}
