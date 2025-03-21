// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 9
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         0       3       0     N/A
//  Average:         0       2       0     N/A
//  Maximum:         0       3       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         0       3       0       0
//  Average:         0       2       0       0
//  Maximum:         0       3       0       0
impl std::ops::Div<UnitizedCenterNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_center_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl std::ops::Div<UnitizedCenterNormPrefixOrPostfix> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_center_norm(self) -> f32 {
        use crate::elements::*;
        self[e321] * self[e321] * self[e4] * self[e4]
    }
}
impl std::ops::Div<UnitizedCenterNormPrefixOrPostfix> for Circle {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_center_norm(self) -> f32 {
        use crate::elements::*;
        self[e423] * self[e423] * self[e321] * self[e321]
    }
}
impl std::ops::Div<UnitizedCenterNormPrefixOrPostfix> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_center_norm(self) -> f32 {
        use crate::elements::*;
        self[e423] * self[e423] * self[e321] * self[e321]
    }
}
impl std::ops::Div<UnitizedCenterNormPrefixOrPostfix> for Dipole {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_center_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl std::ops::Div<UnitizedCenterNormPrefixOrPostfix> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_center_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl std::ops::Div<UnitizedCenterNormPrefixOrPostfix> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for MultiVector {
    fn unitized_center_norm(self) -> f32 {
        0.0
    }
}
impl std::ops::Div<UnitizedCenterNormPrefixOrPostfix> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_center_norm(self) -> f32 {
        use crate::elements::*;
        self[e321] * self[e321] * self[e4] * self[e4]
    }
}
impl std::ops::Div<UnitizedCenterNormPrefixOrPostfix> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_center_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
