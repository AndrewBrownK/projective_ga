// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 9
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
impl std::ops::Div<UnitizedCenterNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for AntiCircleRotor {
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
    fn unitized_center_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
