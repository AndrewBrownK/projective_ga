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
//   Median:         6       9       0
//  Average:         6       7       0
//  Maximum:        31      27       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         6       9       0
//  Average:         6       7       0
//  Maximum:        31      31       1
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        3        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = -self[e23] * self[e23] - self[e31] * self[e31] - self[e12] * self[e12];
        -(self[e41] * self[e41] * anti_dot_product_g0) - (self[e42] * self[e42] * anti_dot_product_g0) - (self[e43] * self[e43] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9        9        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e45] * self[e45]
            - self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12]
            - self[scalar] * self[scalar]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35]);
        -(self[e41] * self[e41] * anti_dot_product_g0) - (self[e42] * self[e42] * anti_dot_product_g0) - (self[e43] * self[e43] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        9        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = -self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12]
            - self[scalar] * self[scalar]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35]);
        -(self[e41] * self[e41] * anti_dot_product_g0) - (self[e42] * self[e42] * anti_dot_product_g0) - (self[e43] * self[e43] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        3        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = -self[scalar] * self[scalar] - self[e23] * self[e23] - self[e31] * self[e31] - self[e12] * self[e12];
        -(self[e41] * self[e41] * anti_dot_product_g0) - (self[e42] * self[e42] * anti_dot_product_g0) - (self[e43] * self[e43] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       12        0
    fn unitized_radius_norm_squared(self) -> f32 {
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
        -(wedge_g0[0] * wedge_g0[0] * anti_dot_product_g0)
            - (wedge_g0[1] * wedge_g0[1] * anti_dot_product_g0)
            - (wedge_g0[2] * wedge_g0[2] * anti_dot_product_g0)
            - (wedge_g0[3] * wedge_g0[3] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        4        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = -self[e321] * self[e321] - self[e1] * self[e1] - self[e2] * self[e2] - self[e3] * self[e3];
        -(self[e423] * self[e423] * anti_dot_product_g0)
            - (self[e431] * self[e431] * anti_dot_product_g0)
            - (self[e412] * self[e412] * anti_dot_product_g0)
            - (self[e4] * self[e4] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       12        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + 2.0 * (self[e5] * self[e4])
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435];
        -(self[e423] * self[e423] * anti_dot_product_g0)
            - (self[e431] * self[e431] * anti_dot_product_g0)
            - (self[e412] * self[e412] * anti_dot_product_g0)
            - (self[e4] * self[e4] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        4        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e321] * self[e321] * -1.0;
        -(self[e423] * self[e423] * anti_dot_product_g0) - (self[e431] * self[e431] * anti_dot_product_g0) - (self[e412] * self[e412] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiDualNum {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiDualNum {
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[scalar] * self[scalar] * f32::powi(self[e1234], -2)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        (self[e1] * self[e1] * f32::powi(self[e4], -2)) + (self[e2] * self[e2] * f32::powi(self[e4], -2)) + (self[e3] * self[e3] * f32::powi(self[e4], -2))
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        4        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = -self[scalar] * self[scalar] - self[e23] * self[e23] - self[e31] * self[e31] - self[e12] * self[e12];
        -(self[e41] * self[e41] * anti_dot_product_g0)
            - (self[e42] * self[e42] * anti_dot_product_g0)
            - (self[e43] * self[e43] * anti_dot_product_g0)
            - (self[e1234] * self[e1234] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for Circle {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        9        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435]
            - self[e321] * self[e321];
        -(self[e423] * self[e423] * anti_dot_product_g0) - (self[e431] * self[e431] * anti_dot_product_g0) - (self[e412] * self[e412] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for CircleAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        9        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435];
        -(self[e423] * self[e423] * anti_dot_product_g0) - (self[e431] * self[e431] * anti_dot_product_g0) - (self[e412] * self[e412] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for CircleAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        9        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235]) + 2.0 * (self[e431] * self[e315]) + 2.0 * (self[e412] * self[e125]);
        -(self[e423] * self[e423] * anti_dot_product_g0) - (self[e431] * self[e431] * anti_dot_product_g0) - (self[e412] * self[e412] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for CircleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        3        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435];
        -(self[e423] * self[e423] * anti_dot_product_g0) - (self[e431] * self[e431] * anti_dot_product_g0) - (self[e412] * self[e412] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235]) + 2.0 * (self[e431] * self[e315]) + 2.0 * (self[e412] * self[e125]) - self[e321] * self[e321];
        -(self[e423] * self[e423] * anti_dot_product_g0) - (self[e431] * self[e431] * anti_dot_product_g0) - (self[e412] * self[e412] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9        9        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435]
            + self[e12345] * self[e12345]
            - self[e321] * self[e321];
        -(self[e423] * self[e423] * anti_dot_product_g0) - (self[e431] * self[e431] * anti_dot_product_g0) - (self[e412] * self[e412] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        9        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435]
            + self[e12345] * self[e12345];
        -(self[e423] * self[e423] * anti_dot_product_g0) - (self[e431] * self[e431] * anti_dot_product_g0) - (self[e412] * self[e412] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        3        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e12345] * self[e12345] + self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435];
        -(self[e423] * self[e423] * anti_dot_product_g0) - (self[e431] * self[e431] * anti_dot_product_g0) - (self[e412] * self[e412] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for Dipole {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        9        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e45] * self[e45]
            - self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35]);
        -(self[e41] * self[e41] * anti_dot_product_g0) - (self[e42] * self[e42] * anti_dot_product_g0) - (self[e43] * self[e43] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e45] * self[e45] - 2.0 * (self[e41] * self[e15]) - 2.0 * (self[e42] * self[e25]) - 2.0 * (self[e43] * self[e35]);
        -(self[e41] * self[e41] * anti_dot_product_g0) - (self[e42] * self[e42] * anti_dot_product_g0) - (self[e43] * self[e43] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for DipoleAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        9        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = -2.0 * (self[e41] * self[e15]) - 2.0 * (self[e42] * self[e25]) - 2.0 * (self[e43] * self[e35]);
        -(self[e41] * self[e41] * anti_dot_product_g0) - (self[e42] * self[e42] * anti_dot_product_g0) - (self[e43] * self[e43] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       12        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125]
            - self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35])
            - 2.0 * (self[e1234] * self[e3215]);
        -(self[e41] * self[e41] * anti_dot_product_g0)
            - (self[e42] * self[e42] * anti_dot_product_g0)
            - (self[e43] * self[e43] * anti_dot_product_g0)
            - (self[e1234] * self[e1234] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       12        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35])
            - 2.0 * (self[e1234] * self[e3215]);
        -(self[e41] * self[e41] * anti_dot_product_g0)
            - (self[e42] * self[e42] * anti_dot_product_g0)
            - (self[e43] * self[e43] * anti_dot_product_g0)
            - (self[e1234] * self[e1234] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6       12        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = -2.0 * (self[e41] * self[e15]) - 2.0 * (self[e42] * self[e25]) - 2.0 * (self[e43] * self[e35]) - 2.0 * (self[e3215] * self[e1234]);
        -(self[e41] * self[e41] * anti_dot_product_g0)
            - (self[e42] * self[e42] * anti_dot_product_g0)
            - (self[e43] * self[e43] * anti_dot_product_g0)
            - (self[e1234] * self[e1234] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        4        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125];
        -(self[e41] * self[e41] * anti_dot_product_g0)
            - (self[e42] * self[e42] * anti_dot_product_g0)
            - (self[e43] * self[e43] * anti_dot_product_g0)
            - (self[e1234] * self[e1234] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       12        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = -self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35])
            - 2.0 * (self[e3215] * self[e1234]);
        -(self[e41] * self[e41] * anti_dot_product_g0)
            - (self[e42] * self[e42] * anti_dot_product_g0)
            - (self[e43] * self[e43] * anti_dot_product_g0)
            - (self[e1234] * self[e1234] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for DipoleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e45] * self[e45];
        -(self[e41] * self[e41] * anti_dot_product_g0) - (self[e42] * self[e42] * anti_dot_product_g0) - (self[e43] * self[e43] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        9        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 =
            -self[e23] * self[e23] - self[e31] * self[e31] - self[e12] * self[e12] - 2.0 * (self[e41] * self[e15]) - 2.0 * (self[e42] * self[e25]) - 2.0 * (self[e43] * self[e35]);
        -(self[e41] * self[e41] * anti_dot_product_g0) - (self[e42] * self[e42] * anti_dot_product_g0) - (self[e43] * self[e43] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for DualNum {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e12345] * self[e12345] * f32::powi(self[e4], -2) * -1.0
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       25        0
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       31       27        0
    //  no simd       31       31        0
    fn unitized_radius_norm_squared(self) -> f32 {
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
        (wedge_g0[0] * wedge_g0[0] * anti_dot_product_g0)
            - (wedge_g0[1] * wedge_g0[1] * anti_dot_product_g0)
            - (wedge_g9[1] * wedge_g9[1] * anti_dot_product_g0)
            - (wedge_g9[2] * wedge_g9[2] * anti_dot_product_g0)
            - (wedge_g9[3] * wedge_g9[3] * anti_dot_product_g0)
            - (self[e4] * self[e4] * anti_dot_product_g0)
            - (self[e41] * self[e41] * anti_dot_product_g0)
            - (self[e42] * self[e42] * anti_dot_product_g0)
            - (self[e43] * self[e43] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for RoundPoint {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        (self[e1] * self[e1] * f32::powi(self[e4], -2)) + (self[e2] * self[e2] * f32::powi(self[e4], -2)) + (self[e3] * self[e3] * f32::powi(self[e4], -2))
            - 2.0 * (self[e5] / (self[e4]))
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for RoundPointAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e5] / (self[e4]) * -2.0
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for Sphere {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        2.0 * (self[e3215] / (self[e1234]))
            - (self[e4235] * self[e4235] * f32::powi(self[e1234], -2))
            - (self[e4315] * self[e4315] * f32::powi(self[e1234], -2))
            - (self[e4125] * self[e4125] * f32::powi(self[e1234], -2))
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for SphereAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e3215] / (self[e1234]) * 2.0
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for SphereOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        -(self[e4235] * self[e4235] * f32::powi(self[e1234], -2))
            - (self[e4315] * self[e4315] * f32::powi(self[e1234], -2))
            - (self[e4125] * self[e4125] * f32::powi(self[e1234], -2))
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       12        0
    fn unitized_radius_norm_squared(self) -> f32 {
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
        -(self[e423] * self[e423] * anti_dot_product_g0)
            - (self[e431] * self[e431] * anti_dot_product_g0)
            - (self[e412] * self[e412] * anti_dot_product_g0)
            - (self[e4] * self[e4] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       12        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + 2.0 * (self[e4] * self[e5])
            + self[e12345] * self[e12345]
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435];
        -(self[e423] * self[e423] * anti_dot_product_g0)
            - (self[e431] * self[e431] * anti_dot_product_g0)
            - (self[e412] * self[e412] * anti_dot_product_g0)
            - (self[e4] * self[e4] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6       12        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235]) + 2.0 * (self[e431] * self[e315]) + 2.0 * (self[e412] * self[e125]) + 2.0 * (self[e4] * self[e5]);
        let wedge_g0 = self.group0().wxyz();
        -(wedge_g0[0] * wedge_g0[0] * anti_dot_product_g0)
            - (wedge_g0[1] * wedge_g0[1] * anti_dot_product_g0)
            - (wedge_g0[2] * wedge_g0[2] * anti_dot_product_g0)
            - (wedge_g0[3] * wedge_g0[3] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        4        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e12345] * self[e12345] + self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435];
        -(self[e423] * self[e423] * anti_dot_product_g0)
            - (self[e431] * self[e431] * anti_dot_product_g0)
            - (self[e412] * self[e412] * anti_dot_product_g0)
            - (self[e4] * self[e4] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       12        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235]) + 2.0 * (self[e431] * self[e315]) + 2.0 * (self[e412] * self[e125]) + 2.0 * (self[e5] * self[e4])
            - self[e321] * self[e321]
            - self[e1] * self[e1]
            - self[e2] * self[e2]
            - self[e3] * self[e3];
        -(self[e423] * self[e423] * anti_dot_product_g0)
            - (self[e431] * self[e431] * anti_dot_product_g0)
            - (self[e412] * self[e412] * anti_dot_product_g0)
            - (self[e4] * self[e4] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       12        0
    fn unitized_radius_norm_squared(self) -> f32 {
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
        -(self[e41] * self[e41] * anti_dot_product_g0)
            - (self[e42] * self[e42] * anti_dot_product_g0)
            - (self[e43] * self[e43] * anti_dot_product_g0)
            - (self[e1234] * self[e1234] * anti_dot_product_g0)
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       12        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = -self[scalar] * self[scalar]
            - self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35])
            - 2.0 * (self[e3215] * self[e1234]);
        -(self[e41] * self[e41] * anti_dot_product_g0)
            - (self[e42] * self[e42] * anti_dot_product_g0)
            - (self[e43] * self[e43] * anti_dot_product_g0)
            - (self[e1234] * self[e1234] * anti_dot_product_g0)
    }
}
