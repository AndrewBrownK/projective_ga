// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 11
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         3       1       0
//   Median:         9       6       0
//  Average:        11       6       0
//  Maximum:        30      16       3
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         3       1       0
//   Median:         9       6       0
//  Average:        11       6       0
//  Maximum:        30      16       3
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
        -(self[e41] * self[e41] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e42] * self[e42] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e43] * self[e43] * f32::powf(anti_dot_product_g0, 0.5))
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
        -(self[e423] * self[e423] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e431] * self[e431] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e412] * self[e412] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e4] * self[e4] * f32::powf(anti_dot_product_g0, 0.5))
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
        -(self[e423] * self[e423] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e431] * self[e431] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e412] * self[e412] * f32::powf(anti_dot_product_g0, 0.5))
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
        -(self[e423] * self[e423] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e431] * self[e431] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e412] * self[e412] * f32::powf(anti_dot_product_g0, 0.5))
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
        -(self[e41] * self[e41] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e42] * self[e42] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e43] * self[e43] * f32::powf(anti_dot_product_g0, 0.5))
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
        -(self[e41] * self[e41] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e42] * self[e42] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e43] * self[e43] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e1234] * self[e1234] * f32::powf(anti_dot_product_g0, 0.5))
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
    //      add/sub      mul      div
    // f32       30       16        0
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
            - 2.0 * (self[e15] * self[e41])
            - 2.0 * (self[e25] * self[e42])
            - 2.0 * (self[e35] * self[e43])
            - 2.0 * (self[e3215] * self[e1234]);
        -(self[e4] * self[e4] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e41] * self[e41] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e42] * self[e42] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e43] * self[e43] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e423] * self[e423] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e431] * self[e431] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e412] * self[e412] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e1234] * self[e1234] * f32::powf(anti_dot_product_g0, 0.5))
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
        (self[e1] * self[e1] / (self[e4])) + (self[e2] * self[e2] / (self[e4])) + (self[e3] * self[e3] / (self[e4])) - 2.0 * self[e5]
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
        2.0 * self[e3215] - (self[e4235] * self[e4235] / (self[e1234])) - (self[e4315] * self[e4315] / (self[e1234])) - (self[e4125] * self[e4125] / (self[e1234]))
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
        -(self[e423] * self[e423] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e431] * self[e431] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e412] * self[e412] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e4] * self[e4] * f32::powf(anti_dot_product_g0, 0.5))
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
        -(self[e41] * self[e41] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e42] * self[e42] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e43] * self[e43] * f32::powf(anti_dot_product_g0, 0.5))
            - (self[e1234] * self[e1234] * f32::powf(anti_dot_product_g0, 0.5))
    }
}
