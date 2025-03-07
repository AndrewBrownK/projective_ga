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
//  Maximum:         2       3       3
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         2       3       3
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
        let wedge_g0 = self.group2().xyz();
        return -(wedge_g0[0] * wedge_g0[0] / (self[e45])) - (wedge_g0[1] * wedge_g0[1] / (self[e45])) - (wedge_g0[2] * wedge_g0[2] / (self[e45]));
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
        return self[e415] * self[e415] * f32::powi(self[e5], 2);
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
        return self[e415] * self[e415] * f32::powi(self[e235], 2);
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
        return self[e415] * self[e415] * f32::powi(self[e235], 2);
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
        let wedge_g0 = self.group2();
        return -(wedge_g0[0] * wedge_g0[0] / (self[e45])) - (wedge_g0[1] * wedge_g0[1] / (self[e45])) - (wedge_g0[2] * wedge_g0[2] / (self[e45]));
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
    // f32        0        2        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        return self[e45] * self[e45] * self[e15] * -1.0;
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for DualNum {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        return self[e5] / (self[e12345]);
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
        let wedge_g0 = self.group0().xyz();
        return -(wedge_g0[0] * wedge_g0[0] / (self[e45])) - (wedge_g0[1] * wedge_g0[1] / (self[e45])) - (wedge_g0[2] * wedge_g0[2] / (self[e45]));
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
    // f32        0        2        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        return self[e45] * self[e45] * self[e15] * -1.0;
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
        return self[e415] * self[e415] * f32::powi(self[e235], 2);
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
        return self[e415] * self[e415] * f32::powi(self[e5], 2);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for MultiVector {
    fn unitized_flat_norm(self) -> f32 {
        return 0.0;
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
        return -(self[e4235] * self[e4235] * self[e3215]) - (self[e4315] * self[e4315] * self[e3215]) - (self[e4125] * self[e4125] * self[e3215]);
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
        return -(self[e4235] * self[e4235] * self[e3215]) - (self[e4315] * self[e4315] * self[e3215]) - (self[e4125] * self[e4125] * self[e3215]);
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
        return self[e415] * self[e415] * f32::powi(self[e5], 2);
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
    // f32        0        2        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        return self[e45] * self[e45] * self[e15] * -1.0;
    }
}
