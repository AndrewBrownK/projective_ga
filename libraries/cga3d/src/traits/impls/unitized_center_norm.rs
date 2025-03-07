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
impl std::ops::Div<UnitizedCenterNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn unitized_center_norm(self) -> f32 {
        use crate::elements::*;
        return (self[e41] * self[e41] * f32::powi(self[e23], 2))
            + (self[e41] * self[e41] * f32::powi(self[e31], 2))
            + (self[e41] * self[e41] * f32::powi(self[e12], 2))
            + (self[e41] * self[e41] * f32::powi(self[scalar], 2));
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
    //      add/sub      mul      div
    // f32        3        0        0
    fn unitized_center_norm(self) -> f32 {
        use crate::elements::*;
        return (self[e321] * self[e321] * f32::powi(self[e4], 2))
            + (self[e4] * self[e4] * f32::powi(self[e1], 2))
            + (self[e4] * self[e4] * f32::powi(self[e2], 2))
            + (self[e4] * self[e4] * f32::powi(self[e3], 2));
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
        return self[e423] * self[e423] * f32::powi(self[e321], 2);
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
        return self[e423] * self[e423] * f32::powi(self[e321], 2);
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
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_center_norm(self) -> f32 {
        use crate::elements::*;
        return (self[e41] * self[e41] * f32::powi(self[e23], 2)) + (self[e41] * self[e41] * f32::powi(self[e31], 2)) + (self[e41] * self[e41] * f32::powi(self[e12], 2));
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
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_center_norm(self) -> f32 {
        use crate::elements::*;
        return (self[e41] * self[e41] * f32::powi(self[e23], 2)) + (self[e41] * self[e41] * f32::powi(self[e31], 2)) + (self[e41] * self[e41] * f32::powi(self[e12], 2));
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
        return 0.0;
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
    //      add/sub      mul      div
    // f32        3        0        0
    fn unitized_center_norm(self) -> f32 {
        use crate::elements::*;
        return (self[e321] * self[e321] * f32::powi(self[e4], 2))
            + (self[e1] * self[e1] * f32::powi(self[e4], 2))
            + (self[e2] * self[e2] * f32::powi(self[e4], 2))
            + (self[e3] * self[e3] * f32::powi(self[e4], 2));
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
    //      add/sub      mul      div
    // f32        3        0        0
    fn unitized_center_norm(self) -> f32 {
        use crate::elements::*;
        return (self[e41] * self[e41] * f32::powi(self[scalar], 2))
            + (self[e41] * self[e41] * f32::powi(self[e23], 2))
            + (self[e41] * self[e41] * f32::powi(self[e31], 2))
            + (self[e41] * self[e41] * f32::powi(self[e12], 2));
    }
}
