// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 11
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         0       0       0
//  Maximum:         3       2       3
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         0       0       0
//  Maximum:         3       2       3
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        (f32::powi(self[e41], 3) * self[e15]) * 2.0
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiDipoleInversion {
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        self[e321] * self[e321] * self[e4] * self[e4]
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for Circle {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for Circle {
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        self[e423] * self[e423] * self[e321] * self[e321]
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for CircleRotor {
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        self[e423] * self[e423] * self[e321] * self[e321]
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for Dipole {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        (f32::powi(self[e41], 3) * self[e15]) * 2.0
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        f32::powi(self[e41], 3) * self[e15] * 2.0
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for MultiVector {
    fn unitized_radius_norm(self) -> f32 {
        0.0
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for RoundPoint {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        3
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        (self[e1] * self[e1] / self[e4]) + (self[e2] * self[e2] / self[e4]) + (self[e3] * self[e3] / self[e4]) - 2.0 * self[e5]
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for Sphere {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        3
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        2.0 * self[e3215] - (self[e4235] * self[e4235] / self[e1234]) - (self[e4315] * self[e4315] / self[e1234]) - (self[e4125] * self[e4125] / self[e1234])
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for VersorEven {
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        self[e321] * self[e321] * self[e4] * self[e4]
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        f32::powi(self[e41], 3) * self[e15] * 2.0
    }
}
