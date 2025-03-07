// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 9
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       0       0
//  Average:         1       0       0
//  Maximum:         3       0       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       0       0
//  Average:         1       0       0
//  Maximum:         3       0       0
impl std::ops::Div<UnitizedCenterNormSquaredPrefixOrPostfix> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm_squared()
    }
}
impl UnitizedCenterNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        (self[e41] * self[e41] * f32::powi(self[e23], 2))
            + (self[e41] * self[e41] * f32::powi(self[e31], 2))
            + (self[e41] * self[e41] * f32::powi(self[e12], 2))
            + (self[e41] * self[e41] * f32::powi(self[scalar], 2))
    }
}
impl std::ops::Div<UnitizedCenterNormSquaredPrefixOrPostfix> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm_squared()
    }
}
impl UnitizedCenterNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        let wedge_g0 = self.group0().with_w(self[e4]).wxyz();
        (wedge_g0[0] * wedge_g0[0] * f32::powi(self[e321], 2))
            + (wedge_g0[0] * wedge_g0[0] * f32::powi(self[e1], 2))
            + (wedge_g0[0] * wedge_g0[0] * f32::powi(self[e2], 2))
            + (wedge_g0[0] * wedge_g0[0] * f32::powi(self[e3], 2))
    }
}
impl std::ops::Div<UnitizedCenterNormSquaredPrefixOrPostfix> for Circle {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm_squared()
    }
}
impl UnitizedCenterNormSquared for Circle {
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e423] * self[e423] * f32::powi(self[e321], 2)
    }
}
impl std::ops::Div<UnitizedCenterNormSquaredPrefixOrPostfix> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm_squared()
    }
}
impl UnitizedCenterNormSquared for CircleRotor {
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e423] * self[e423] * f32::powi(self[e321], 2)
    }
}
impl std::ops::Div<UnitizedCenterNormSquaredPrefixOrPostfix> for Dipole {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm_squared()
    }
}
impl UnitizedCenterNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        (self[e41] * self[e41] * f32::powi(self[e23], 2)) + (self[e41] * self[e41] * f32::powi(self[e31], 2)) + (self[e41] * self[e41] * f32::powi(self[e12], 2))
    }
}
impl std::ops::Div<UnitizedCenterNormSquaredPrefixOrPostfix> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm_squared()
    }
}
impl UnitizedCenterNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        (self[e41] * self[e41] * f32::powi(self[e23], 2)) + (self[e41] * self[e41] * f32::powi(self[e31], 2)) + (self[e41] * self[e41] * f32::powi(self[e12], 2))
    }
}
impl std::ops::Div<UnitizedCenterNormSquaredPrefixOrPostfix> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm_squared()
    }
}
impl UnitizedCenterNormSquared for MultiVector {
    fn unitized_center_norm_squared(self) -> f32 {
        0.0
    }
}
impl std::ops::Div<UnitizedCenterNormSquaredPrefixOrPostfix> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm_squared()
    }
}
impl UnitizedCenterNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        (self[e321] * self[e321] * f32::powi(self[e4], 2))
            + (self[e1] * self[e1] * f32::powi(self[e4], 2))
            + (self[e2] * self[e2] * f32::powi(self[e4], 2))
            + (self[e3] * self[e3] * f32::powi(self[e4], 2))
    }
}
impl std::ops::Div<UnitizedCenterNormSquaredPrefixOrPostfix> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm_squared()
    }
}
impl UnitizedCenterNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        (self[e41] * self[e41] * f32::powi(self[scalar], 2))
            + (self[e41] * self[e41] * f32::powi(self[e23], 2))
            + (self[e41] * self[e41] * f32::powi(self[e31], 2))
            + (self[e41] * self[e41] * f32::powi(self[e12], 2))
    }
}
