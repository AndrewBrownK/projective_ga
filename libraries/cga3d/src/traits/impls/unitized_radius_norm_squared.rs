// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 11
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         3      11       0     N/A
//   Median:         9      17       0     N/A
//  Average:        11      20       0     N/A
//  Maximum:        30      48       4     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         3      11       0       0
//   Median:         9      17       0       0
//  Average:        11      20       0       0
//  Maximum:        30      48       4       0
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        9       17        0        0
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
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32       13       23        0        0
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
        -(self[e423] * self[e423] * anti_dot_product_g0)
            - (self[e431] * self[e431] * anti_dot_product_g0)
            - (self[e412] * self[e412] * anti_dot_product_g0)
            - (self[e4] * self[e4] * anti_dot_product_g0)
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
    //      add/sub      mul      div      pow
    // f32        8       16        0        0
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
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        9       17        0        0
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
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for Dipole {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        8       16        0        0
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
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32       13       23        0        0
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
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32       30       48        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + 2.0 * (self[e4] * self[e5])
            + self[e12345] * self[e12345]
            + self[e45] * self[e45]
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435]
            + self[e4235] * self[e4235]
            + self[e4315] * self[e4315]
            + self[e4125] * self[e4125]
            - self[scalar] * self[scalar]
            - self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12]
            - self[e1] * self[e1]
            - self[e2] * self[e2]
            - self[e3] * self[e3]
            - self[e321] * self[e321]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35])
            - 2.0 * (self[e3215] * self[e1234]);
        -(self[e41] * self[e41] * anti_dot_product_g0)
            - (self[e42] * self[e42] * anti_dot_product_g0)
            - (self[e43] * self[e43] * anti_dot_product_g0)
            - (self[e423] * self[e423] * anti_dot_product_g0)
            - (self[e431] * self[e431] * anti_dot_product_g0)
            - (self[e412] * self[e412] * anti_dot_product_g0)
            - (self[e4] * self[e4] * anti_dot_product_g0)
            - (self[e1234] * self[e1234] * anti_dot_product_g0)
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
    //      add/sub      mul      div      pow
    // f32        3       11        4        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        (self[e1] * self[e1] / (self[e4] * self[e4])) + (self[e2] * self[e2] / (self[e4] * self[e4])) + (self[e3] * self[e3] / (self[e4] * self[e4])) - 2.0 * (self[e5] / self[e4])
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
    //      add/sub      mul      div      pow
    // f32        3       11        4        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        2.0 * (self[e3215] / self[e1234])
            - (self[e4235] * self[e4235] / (self[e1234] * self[e1234]))
            - (self[e4315] * self[e4315] / (self[e1234] * self[e1234]))
            - (self[e4125] * self[e4125] / (self[e1234] * self[e1234]))
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
    //      add/sub      mul      div      pow
    // f32       14       24        0        0
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
        let wedge_g1_xyz = self.group0().xyz();
        -(wedge_g1_xyz[0] * wedge_g1_xyz[0] * anti_dot_product_g0)
            - (wedge_g1_xyz[1] * wedge_g1_xyz[1] * anti_dot_product_g0)
            - (wedge_g1_xyz[2] * wedge_g1_xyz[2] * anti_dot_product_g0)
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
    //      add/sub      mul      div      pow
    // f32       14       24        0        0
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
        let sub_type_g0 = self.group0().xyz();
        -(sub_type_g0[0] * sub_type_g0[0] * anti_dot_product_g0)
            - (sub_type_g0[1] * sub_type_g0[1] * anti_dot_product_g0)
            - (sub_type_g0[2] * sub_type_g0[2] * anti_dot_product_g0)
            - (self[e1234] * self[e1234] * anti_dot_product_g0)
    }
}
