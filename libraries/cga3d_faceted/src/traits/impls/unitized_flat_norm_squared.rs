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
//   Median:         0       2       0
//  Average:         0       1       0
//  Maximum:         2      13       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       4       0
//  Average:         0       3       0
//  Maximum:         2      19       0
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group2().xyz());
        let sub_type_2 = FlatOrigin::from_groups(/* e45 */ self[e45]);
        return -(f32::powi(wedge[e415], 2) * f32::powi(sub_type_2[e45], -2))
            - (f32::powi(wedge[e425], 2) * f32::powi(sub_type_2[e45], -2))
            - (f32::powi(wedge[e435], 2) * f32::powi(sub_type_2[e45], -2));
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1().xyz());
        let sub_type_2 = FlatOrigin::from_groups(/* e45 */ self[e45]);
        return -(f32::powi(wedge[e415], 2) * f32::powi(sub_type_2[e45], -2))
            - (f32::powi(wedge[e425], 2) * f32::powi(sub_type_2[e45], -2))
            - (f32::powi(wedge[e435], 2) * f32::powi(sub_type_2[e45], -2));
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self.group1().xyz()[0], 2) * f32::powi((Simd32x4::from([self[e5], self[e235], self[e315], self[e125]]) * Simd32x4::from(-1.0))[0], 2) * -1.0;
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self.group0().xyz()[0], 2) * f32::powi((self.group1().with_w(self[e5]).wxyz() * Simd32x4::from(-1.0))[0], 2) * -1.0;
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi((Simd32x4::from([self[e5], self[e235], self[e315], self[e125]]) * Simd32x4::from(-1.0))[0], 2) * f32::powi(self[e415], 2) * -1.0;
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for Circle {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn unitized_flat_norm_squared(self) -> f32 {
        return f32::powi((self.group2() * Simd32x3::from(-1.0))[0], 2) * f32::powi(self.group1().xyz()[0], 2) * -1.0;
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for CircleAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi((self.group2() * Simd32x3::from(-1.0))[0], 2) * f32::powi(self[e415], 2) * -1.0;
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for CircleAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn unitized_flat_norm_squared(self) -> f32 {
        return f32::powi((self.group1() * Simd32x3::from(-1.0))[0], 2) * f32::powi(self.group0().xyz()[0], 2) * -1.0;
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi((self.group2().xyz() * Simd32x3::from(-1.0))[0], 2) * f32::powi(self[e415], 2) * -1.0;
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi((self.group2().xyz() * Simd32x3::from(-1.0))[0], 2) * f32::powi(self.group1().with_w(self[e12345])[0], 2) * -1.0;
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi((self.group1().xyz() * Simd32x3::from(-1.0))[0], 2) * f32::powi(self.group0().with_w(self[e12345])[0], 2) * -1.0;
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi((self.group1().xyz() * Simd32x3::from(-1.0))[0], 2) * f32::powi(self[e415], 2) * -1.0;
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for Dipole {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group2());
        let sub_type_2 = FlatOrigin::from_groups(/* e45 */ self[e45]);
        return -(f32::powi(wedge[e415], 2) * f32::powi(sub_type_2[e45], -2))
            - (f32::powi(wedge[e425], 2) * f32::powi(sub_type_2[e45], -2))
            - (f32::powi(wedge[e435], 2) * f32::powi(sub_type_2[e45], -2));
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1());
        let sub_type_2 = FlatOrigin::from_groups(/* e45 */ self[e45]);
        return -(f32::powi(wedge[e415], 2) * f32::powi(sub_type_2[e45], -2))
            - (f32::powi(wedge[e425], 2) * f32::powi(sub_type_2[e45], -2))
            - (f32::powi(wedge[e435], 2) * f32::powi(sub_type_2[e45], -2));
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for DipoleAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1());
        let sub_type_2 = FlatOrigin::from_groups(/* e45 */ self[e45]);
        return -(f32::powi(wedge[e415], 2) * f32::powi(sub_type_2[e45], -2))
            - (f32::powi(wedge[e425], 2) * f32::powi(sub_type_2[e45], -2))
            - (f32::powi(wedge[e435], 2) * f32::powi(sub_type_2[e45], -2));
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e45], 2) * f32::powi(self[e15], 2) * -1.0;
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e45], 2) * f32::powi(self[e15], 2) * -1.0;
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self.group1().with_w(self[e3215])[0], 2) * f32::powi(self[e45], 2) * -1.0;
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for FlatPoint {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz());
        let sub_type_2 = FlatOrigin::from_groups(/* e45 */ self[e45]);
        return -(f32::powi(wedge[e415], 2) * f32::powi(sub_type_2[e45], -2))
            - (f32::powi(wedge[e425], 2) * f32::powi(sub_type_2[e45], -2))
            - (f32::powi(wedge[e435], 2) * f32::powi(sub_type_2[e45], -2));
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for Flector {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e15], 2) * f32::powi(self[e45], 2) * -1.0;
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for Line {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi((self.group1() * Simd32x3::from(-1.0))[0], 2) * f32::powi(self[e415], 2) * -1.0;
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for Motor {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi((self.group1().wxyz() * Simd32x4::from(-1.0))[0], 2) * f32::powi(self[e415], 2) * -1.0;
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       10        0
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0       13        0
    //  no simd        0       19        0
    fn unitized_flat_norm_squared(self) -> f32 {
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
        return sub_type_2[e423] * sub_type_2[e235] * wedge[e41] * wedge[e15] * 4.0;
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for Plane {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        let wedge = AntiScalar::from_groups(/* e12345 */ self[e3215]);
        let sub_type_2 = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz());
        return -(f32::powi(wedge[e12345], 2) * f32::powi(sub_type_2[e4235], 2))
            - (f32::powi(wedge[e12345], 2) * f32::powi(sub_type_2[e4315], 2))
            - (f32::powi(wedge[e12345], 2) * f32::powi(sub_type_2[e4125], 2));
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for Sphere {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        let wedge = AntiScalar::from_groups(/* e12345 */ self[e3215]);
        let sub_type_2 = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz());
        return -(f32::powi(wedge[e12345], 2) * f32::powi(sub_type_2[e4235], 2))
            - (f32::powi(wedge[e12345], 2) * f32::powi(sub_type_2[e4315], 2))
            - (f32::powi(wedge[e12345], 2) * f32::powi(sub_type_2[e4125], 2));
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi((self.group2().wxyz() * Simd32x4::from(-1.0))[0], 2) * f32::powi(self[e415], 2) * -1.0;
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi((self.group2().wxyz() * Simd32x4::from(-1.0))[0], 2) * f32::powi(self[e415], 2) * -1.0;
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi((self.group2().wxyz() * Simd32x4::from(-1.0))[0], 2) * f32::powi(self[e415], 2) * -1.0;
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e45], 2) * f32::powi(self[e15], 2) * -1.0;
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e15], 2) * f32::powi(self[e45], 2) * -1.0;
    }
}
