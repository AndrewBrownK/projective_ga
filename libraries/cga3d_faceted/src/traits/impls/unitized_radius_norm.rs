// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 43
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         6       6       0
//  Average:         6       4       0
//  Maximum:        31      18       3
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         6       6       0
//  Average:         6       4       0
//  Maximum:        31      22       3
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = -self[e23] * self[e23] - self[e31] * self[e31] - self[e12] * self[e12];
        return -(self[e41] * self[e41] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e42] * self[e42] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e43] * self[e43] * f32::powf(anti_dot_product_g0, 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9        6        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e45] * self[e45]
            - self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12]
            - self[scalar] * self[scalar]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35]);
        return -(self[e41] * self[e41] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e42] * self[e42] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e43] * self[e43] * f32::powf(anti_dot_product_g0, 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        6        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = -self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12]
            - self[scalar] * self[scalar]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35]);
        return -(self[e41] * self[e41] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e42] * self[e42] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e43] * self[e43] * f32::powf(anti_dot_product_g0, 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = -self[scalar] * self[scalar] - self[e23] * self[e23] - self[e31] * self[e31] - self[e12] * self[e12];
        return -(self[e41] * self[e41] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e42] * self[e42] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e43] * self[e43] * f32::powf(anti_dot_product_g0, 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13        8        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + 2.0 * (self[e4] * self[e5])
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435]
            - self[e321] * self[e321]
            - self[e1] * self[e1]
            - self[e2] * self[e2]
            - self[e3] * self[e3];
        let wedge_g0 = self.group0().with_w(self[e4]).wxyz();
        return -(wedge_g0[0] * wedge_g0[0] * f32::powf(anti_dot_product_g0, 0.5))
            - (wedge_g0[1] * wedge_g0[1] * f32::powf(anti_dot_product_g0, 0.5))
            - (wedge_g0[2] * wedge_g0[2] * f32::powf(anti_dot_product_g0, 0.5))
            - (wedge_g0[3] * wedge_g0[3] * f32::powf(anti_dot_product_g0, 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = -self[e321] * self[e321] - self[e1] * self[e1] - self[e2] * self[e2] - self[e3] * self[e3];
        return -(self[e423] * self[e423] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e431] * self[e431] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e412] * self[e412] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e4] * self[e4] * f32::powf(anti_dot_product_g0, 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9        8        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + 2.0 * (self[e5] * self[e4])
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435];
        return -(self[e423] * self[e423] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e431] * self[e431] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e412] * self[e412] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e4] * self[e4] * f32::powf(anti_dot_product_g0, 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e321] * self[e321] * -1.0;
        return -(self[e423] * self[e423] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e431] * self[e431] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e412] * self[e412] * f32::powf(anti_dot_product_g0, 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiDualNum {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        return self[scalar] / (self[e1234]);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        3
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        return (self[e1] * self[e1] / (self[e4])) + (self[e2] * self[e2] / (self[e4])) + (self[e3] * self[e3] / (self[e4]));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = -self[scalar] * self[scalar] - self[e23] * self[e23] - self[e31] * self[e31] - self[e12] * self[e12];
        return -(self[e41] * self[e41] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e42] * self[e42] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e43] * self[e43] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e1234] * self[e1234] * f32::powf(anti_dot_product_g0, 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for Circle {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        6        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435]
            - self[e321] * self[e321];
        return -(self[e423] * self[e423] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e431] * self[e431] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e412] * self[e412] * f32::powf(anti_dot_product_g0, 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for CircleAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        6        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435];
        return -(self[e423] * self[e423] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e431] * self[e431] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e412] * self[e412] * f32::powf(anti_dot_product_g0, 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for CircleAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        6        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235]) + 2.0 * (self[e431] * self[e315]) + 2.0 * (self[e412] * self[e125]);
        return -(self[e423] * self[e423] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e431] * self[e431] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e412] * self[e412] * f32::powf(anti_dot_product_g0, 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for CircleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435];
        return -(self[e423] * self[e423] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e431] * self[e431] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e412] * self[e412] * f32::powf(anti_dot_product_g0, 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235]) + 2.0 * (self[e431] * self[e315]) + 2.0 * (self[e412] * self[e125]) - self[e321] * self[e321];
        return -(self[e423] * self[e423] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e431] * self[e431] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e412] * self[e412] * f32::powf(anti_dot_product_g0, 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9        6        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435]
            + self[e12345] * self[e12345]
            - self[e321] * self[e321];
        return -(self[e423] * self[e423] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e431] * self[e431] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e412] * self[e412] * f32::powf(anti_dot_product_g0, 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        6        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435]
            + self[e12345] * self[e12345];
        return -(self[e423] * self[e423] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e431] * self[e431] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e412] * self[e412] * f32::powf(anti_dot_product_g0, 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e12345] * self[e12345] + self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435];
        return -(self[e423] * self[e423] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e431] * self[e431] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e412] * self[e412] * f32::powf(anti_dot_product_g0, 0.5));
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
    // f32        8        6        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e45] * self[e45]
            - self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35]);
        return -(self[e41] * self[e41] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e42] * self[e42] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e43] * self[e43] * f32::powf(anti_dot_product_g0, 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e45] * self[e45] - 2.0 * (self[e41] * self[e15]) - 2.0 * (self[e42] * self[e25]) - 2.0 * (self[e43] * self[e35]);
        return -(self[e41] * self[e41] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e42] * self[e42] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e43] * self[e43] * f32::powf(anti_dot_product_g0, 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for DipoleAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        6        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = -2.0 * (self[e41] * self[e15]) - 2.0 * (self[e42] * self[e25]) - 2.0 * (self[e43] * self[e35]);
        return -(self[e41] * self[e41] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e42] * self[e42] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e43] * self[e43] * f32::powf(anti_dot_product_g0, 0.5));
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
    // f32       13        8        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125]
            - self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35])
            - 2.0 * (self[e1234] * self[e3215]);
        return -(self[e41] * self[e41] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e42] * self[e42] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e43] * self[e43] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e1234] * self[e1234] * f32::powf(anti_dot_product_g0, 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        8        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35])
            - 2.0 * (self[e1234] * self[e3215]);
        return -(self[e41] * self[e41] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e42] * self[e42] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e43] * self[e43] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e1234] * self[e1234] * f32::powf(anti_dot_product_g0, 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        8        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = -2.0 * (self[e41] * self[e15]) - 2.0 * (self[e42] * self[e25]) - 2.0 * (self[e43] * self[e35]) - 2.0 * (self[e3215] * self[e1234]);
        return -(self[e41] * self[e41] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e42] * self[e42] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e43] * self[e43] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e1234] * self[e1234] * f32::powf(anti_dot_product_g0, 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125];
        return -(self[e41] * self[e41] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e42] * self[e42] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e43] * self[e43] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e1234] * self[e1234] * f32::powf(anti_dot_product_g0, 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9        8        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = -self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35])
            - 2.0 * (self[e3215] * self[e1234]);
        return -(self[e41] * self[e41] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e42] * self[e42] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e43] * self[e43] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e1234] * self[e1234] * f32::powf(anti_dot_product_g0, 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for DipoleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e45] * self[e45];
        return -(self[e41] * self[e41] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e42] * self[e42] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e43] * self[e43] * f32::powf(anti_dot_product_g0, 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        6        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 =
            -self[e23] * self[e23] - self[e31] * self[e31] - self[e12] * self[e12] - 2.0 * (self[e41] * self[e15]) - 2.0 * (self[e42] * self[e25]) - 2.0 * (self[e43] * self[e35]);
        return -(self[e41] * self[e41] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e42] * self[e42] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e43] * self[e43] * f32::powf(anti_dot_product_g0, 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for DualNum {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        return self[e12345] / (self[e4]) * -1.0;
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       16        0
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       31       18        0
    //  no simd       31       22        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e4] * self[e5])
            + 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + self[e12345] * self[e12345]
            + self[e45] * self[e45]
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435]
            + self[e4235] * self[e4235]
            + self[e4315] * self[e4315]
            + self[e4125] * self[e4125]
            - self[scalar] * self[scalar]
            - self[e1] * self[e1]
            - self[e2] * self[e2]
            - self[e3] * self[e3]
            - self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12]
            - self[e321] * self[e321]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35])
            - 2.0 * (self[e1234] * self[e3215]);
        let wedge_g0 = Simd32x2::from([1.0, self[e1234]]) * Simd32x2::from([0.0, 1.0]);
        let wedge_g9 = Simd32x4::from([0.0, self[e423], self[e431], self[e412]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]);
        return (wedge_g0[0] * wedge_g0[0] * f32::powf(anti_dot_product_g0, 0.5))
            - (wedge_g0[1] * wedge_g0[1] * f32::powf(anti_dot_product_g0, 0.5))
            - (wedge_g9[1] * wedge_g9[1] * f32::powf(anti_dot_product_g0, 0.5))
            - (wedge_g9[2] * wedge_g9[2] * f32::powf(anti_dot_product_g0, 0.5))
            - (wedge_g9[3] * wedge_g9[3] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e4] * self[e4] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e41] * self[e41] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e42] * self[e42] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e43] * self[e43] * f32::powf(anti_dot_product_g0, 0.5));
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
        return (self[e1] * self[e1] / (self[e4])) + (self[e2] * self[e2] / (self[e4])) + (self[e3] * self[e3] / (self[e4])) - 2.0 * self[e5];
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for RoundPointAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powf(self[e5], 0.5) * f32::powf(self[e4], -0.5) * -2.0;
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
        return 2.0 * self[e3215] - (self[e4235] * self[e4235] / (self[e1234])) - (self[e4315] * self[e4315] / (self[e1234])) - (self[e4125] * self[e4125] / (self[e1234]));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for SphereAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powf(self[e3215], 0.5) * f32::powf(self[e1234], -0.5) * 2.0;
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for SphereOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        3
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        return -(self[e4235] * self[e4235] / (self[e1234])) - (self[e4315] * self[e4315] / (self[e1234])) - (self[e4125] * self[e4125] / (self[e1234]));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14        8        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + 2.0 * (self[e5] * self[e4])
            + self[e12345] * self[e12345]
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435]
            - self[e321] * self[e321]
            - self[e1] * self[e1]
            - self[e2] * self[e2]
            - self[e3] * self[e3];
        return -(self[e423] * self[e423] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e431] * self[e431] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e412] * self[e412] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e4] * self[e4] * f32::powf(anti_dot_product_g0, 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        8        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + 2.0 * (self[e4] * self[e5])
            + self[e12345] * self[e12345]
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435];
        return -(self[e423] * self[e423] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e431] * self[e431] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e412] * self[e412] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e4] * self[e4] * f32::powf(anti_dot_product_g0, 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        8        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235]) + 2.0 * (self[e431] * self[e315]) + 2.0 * (self[e412] * self[e125]) + 2.0 * (self[e4] * self[e5]);
        let wedge_g0 = self.group0().wxyz();
        return -(wedge_g0[0] * wedge_g0[0] * f32::powf(anti_dot_product_g0, 0.5))
            - (wedge_g0[1] * wedge_g0[1] * f32::powf(anti_dot_product_g0, 0.5))
            - (wedge_g0[2] * wedge_g0[2] * f32::powf(anti_dot_product_g0, 0.5))
            - (wedge_g0[3] * wedge_g0[3] * f32::powf(anti_dot_product_g0, 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e12345] * self[e12345] + self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435];
        return -(self[e423] * self[e423] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e431] * self[e431] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e412] * self[e412] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e4] * self[e4] * f32::powf(anti_dot_product_g0, 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        8        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235]) + 2.0 * (self[e431] * self[e315]) + 2.0 * (self[e412] * self[e125]) + 2.0 * (self[e5] * self[e4])
            - self[e321] * self[e321]
            - self[e1] * self[e1]
            - self[e2] * self[e2]
            - self[e3] * self[e3];
        return -(self[e423] * self[e423] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e431] * self[e431] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e412] * self[e412] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e4] * self[e4] * f32::powf(anti_dot_product_g0, 0.5));
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
    // f32       14        8        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125]
            - self[scalar] * self[scalar]
            - self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35])
            - 2.0 * (self[e1234] * self[e3215]);
        return -(self[e41] * self[e41] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e42] * self[e42] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e43] * self[e43] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e1234] * self[e1234] * f32::powf(anti_dot_product_g0, 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        8        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = -self[scalar] * self[scalar]
            - self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35])
            - 2.0 * (self[e3215] * self[e1234]);
        return -(self[e41] * self[e41] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e42] * self[e42] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e43] * self[e43] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e1234] * self[e1234] * f32::powf(anti_dot_product_g0, 0.5));
    }
}
