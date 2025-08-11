// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 16
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         0       3       0     N/A
//  Average:         0       4       0     N/A
//  Maximum:         2       9       3     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         0       3       0       0
//  Average:         0       4       0       0
//  Maximum:         2       9       3       0
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        9        3        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        let wedge_g0 = self.group2().xyz();
        -(wedge_g0[0] * wedge_g0[0] / (self[e45] * self[e45])) - (wedge_g0[1] * wedge_g0[1] / (self[e45] * self[e45])) - (wedge_g0[2] * wedge_g0[2] / (self[e45] * self[e45]))
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
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e5] * self[e5]
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
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e235] * self[e235] * self[e415] * self[e415]
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
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e235] * self[e235]
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
    //      add/sub      mul      div      pow
    // f32        2        9        3        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        -(self[e15] * self[e15] / (self[e45] * self[e45])) - (self[e25] * self[e25] / (self[e45] * self[e45])) - (self[e35] * self[e35] / (self[e45] * self[e45]))
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
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e45] * self[e45] * self[e15] * -1.0
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for DualNum {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e5] * self[e5] / (self[e12345] * self[e12345])
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
    //      add/sub      mul      div      pow
    // f32        2        9        3        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        let wedge_g0 = self.group0().xyz();
        -(wedge_g0[0] * wedge_g0[0] / (self[e45] * self[e45])) - (wedge_g0[1] * wedge_g0[1] / (self[e45] * self[e45])) - (wedge_g0[2] * wedge_g0[2] / (self[e45] * self[e45]))
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
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e45] * self[e45] * self[e15] * -1.0
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
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e235] * self[e235]
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
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e5] * self[e5]
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
    //      add/sub      mul      div      pow
    // f32        2        9        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type_g0_xyz = self.group0().xyz();
        -(sub_type_g0_xyz[0] * sub_type_g0_xyz[0] * self[e3215] * self[e3215])
            - (sub_type_g0_xyz[1] * sub_type_g0_xyz[1] * self[e3215] * self[e3215])
            - (sub_type_g0_xyz[2] * sub_type_g0_xyz[2] * self[e3215] * self[e3215])
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
    //      add/sub      mul      div      pow
    // f32        2        9        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type_g0_xyz = self.group0().xyz();
        -(sub_type_g0_xyz[0] * sub_type_g0_xyz[0] * self[e3215] * self[e3215])
            - (sub_type_g0_xyz[1] * sub_type_g0_xyz[1] * self[e3215] * self[e3215])
            - (sub_type_g0_xyz[2] * sub_type_g0_xyz[2] * self[e3215] * self[e3215])
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
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e5] * self[e5]
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
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e45] * self[e45] * self[e15] * -1.0
    }
}
