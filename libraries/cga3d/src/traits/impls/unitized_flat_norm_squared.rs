// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 16
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         2       3       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         2       3       0
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
        let wedge_g0 = self.group2().xyz();
        -(wedge_g0[0] * wedge_g0[0] * f32::powi(self[e45], -2)) - (wedge_g0[1] * wedge_g0[1] * f32::powi(self[e45], -2)) - (wedge_g0[2] * wedge_g0[2] * f32::powi(self[e45], -2))
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for AntiDipoleInversion {
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * f32::powi(self[e5], 2)
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for Circle {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Circle {
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * f32::powi(self[e235], 2)
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for CircleRotor {
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * f32::powi(self[e235], 2)
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
        let wedge_g0 = self.group2();
        -(wedge_g0[0] * wedge_g0[0] * f32::powi(self[e45], -2)) - (wedge_g0[1] * wedge_g0[1] * f32::powi(self[e45], -2)) - (wedge_g0[2] * wedge_g0[2] * f32::powi(self[e45], -2))
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
    // f32        0        3        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e45] * self[e45] * Simd32x3::from(0.0).with_w(1.0).wwwx()[0] * self[e15] * -1.0
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for DualNum {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for DualNum {
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e5] * self[e5] * f32::powi(self[e12345], -2)
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
        let wedge_g0 = self.group0().xyz();
        -(wedge_g0[0] * wedge_g0[0] * f32::powi(self[e45], -2)) - (wedge_g0[1] * wedge_g0[1] * f32::powi(self[e45], -2)) - (wedge_g0[2] * wedge_g0[2] * f32::powi(self[e45], -2))
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
    // f32        0        3        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e45] * self[e45] * Simd32x3::from(0.0).with_w(1.0).wwwx()[0] * self[e15] * -1.0
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for Line {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Line {
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * f32::powi(self[e235], 2)
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for Motor {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Motor {
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * f32::powi(self[e5], 2)
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for MultiVector {
    fn unitized_flat_norm_squared(self) -> f32 {
        0.0
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
        -(self[e4235] * self[e4235] * f32::powi(self[e3215], 2)) - (self[e4315] * self[e4315] * f32::powi(self[e3215], 2)) - (self[e4125] * self[e4125] * f32::powi(self[e3215], 2))
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
        -(self[e4235] * self[e4235] * f32::powi(self[e3215], 2)) - (self[e4315] * self[e4315] * f32::powi(self[e3215], 2)) - (self[e4125] * self[e4125] * f32::powi(self[e3215], 2))
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for VersorEven {
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * f32::powi(self[e5], 2)
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
    // f32        0        3        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e45] * self[e45] * Simd32x3::from(0.0).with_w(1.0).wwwx()[0] * self[e15] * -1.0
    }
}
