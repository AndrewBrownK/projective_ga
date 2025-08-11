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
//  Minimum:         3       4       0     N/A
//   Median:         6       7       0     N/A
//  Average:         6       7       0     N/A
//  Maximum:        15      16       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         3       4       0       0
//   Median:         6       7       0       0
//  Average:         6       7       0       0
//  Maximum:        15      16       0       0
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for AntiCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4        5        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g0_xyz = self.group1().xyz();
        Scalar::from_groups(
            // scalar
            sub_type_g0_xyz[0] * sub_type_g0_xyz[0] + sub_type_g0_xyz[1] * sub_type_g0_xyz[1] + sub_type_g0_xyz[2] * sub_type_g0_xyz[2] + self[scalar] * self[scalar]
                - self[e45] * self[e45],
        )
    }
}
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for AntiDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        6        7        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g1_xyz = self.group3().xyz();
        let sub_type_g0 = self.group1().xyz();
        Scalar::from_groups(
            // scalar
            sub_type_g1_xyz[0] * sub_type_g1_xyz[0] + sub_type_g1_xyz[1] * sub_type_g1_xyz[1] + sub_type_g1_xyz[2] * sub_type_g1_xyz[2] + self[e321] * self[e321]
                - sub_type_g0[0] * sub_type_g0[0]
                - sub_type_g0[1] * sub_type_g0[1]
                - sub_type_g0[2] * sub_type_g0[2],
        )
    }
}
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for Circle {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g0 = self.group1().xyz();
        Scalar::from_groups(
            // scalar
            self[e321] * self[e321] - sub_type_g0[0] * sub_type_g0[0] - sub_type_g0[1] * sub_type_g0[1] - sub_type_g0[2] * sub_type_g0[2],
        )
    }
}
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for CircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4        5        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g0_xyz = self.group1().xyz();
        Scalar::from_groups(
            // scalar
            self[e321] * self[e321]
                - sub_type_g0_xyz[0] * sub_type_g0_xyz[0]
                - sub_type_g0_xyz[1] * sub_type_g0_xyz[1]
                - sub_type_g0_xyz[2] * sub_type_g0_xyz[2]
                - self[e12345] * self[e12345],
        )
    }
}
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for Dipole {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g0 = self.group1().xyz();
        Scalar::from_groups(
            // scalar
            sub_type_g0[0] * sub_type_g0[0] + sub_type_g0[1] * sub_type_g0[1] + sub_type_g0[2] * sub_type_g0[2] - self[e45] * self[e45],
        )
    }
}
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for DipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        6        7        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g0 = self.group1().xyz();
        let sub_type_g1_xyz = self.group3().xyz();
        Scalar::from_groups(
            // scalar
            sub_type_g0[0] * sub_type_g0[0] + sub_type_g0[1] * sub_type_g0[1] + sub_type_g0[2] * sub_type_g0[2]
                - sub_type_g1_xyz[0] * sub_type_g1_xyz[0]
                - sub_type_g1_xyz[1] * sub_type_g1_xyz[1]
                - sub_type_g1_xyz[2] * sub_type_g1_xyz[2]
                - self[e45] * self[e45],
        )
    }
}
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for MultiVector {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32       15       16        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g1_xyz = self.group1().xyz();
        let sub_type_g6_xyz = self.group6().xyz();
        let sub_type_g9_xyz = self.group9().xyz();
        Scalar::from_groups(
            // scalar
            self[scalar] * self[scalar]
                + sub_type_g1_xyz[0] * sub_type_g1_xyz[0]
                + sub_type_g1_xyz[1] * sub_type_g1_xyz[1]
                + sub_type_g1_xyz[2] * sub_type_g1_xyz[2]
                + self[e23] * self[e23]
                + self[e31] * self[e31]
                + self[e12] * self[e12]
                + self[e321] * self[e321]
                - self[e12345] * self[e12345]
                - sub_type_g6_xyz[0] * sub_type_g6_xyz[0]
                - sub_type_g6_xyz[1] * sub_type_g6_xyz[1]
                - sub_type_g6_xyz[2] * sub_type_g6_xyz[2]
                - sub_type_g9_xyz[0] * sub_type_g9_xyz[0]
                - sub_type_g9_xyz[1] * sub_type_g9_xyz[1]
                - sub_type_g9_xyz[2] * sub_type_g9_xyz[2]
                - self[e45] * self[e45],
        )
    }
}
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for VersorEven {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        7        8        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g1_xyz = self.group3().xyz();
        let sub_type_g0_xyz = self.group1().xyz();
        Scalar::from_groups(
            // scalar
            sub_type_g1_xyz[0] * sub_type_g1_xyz[0] + sub_type_g1_xyz[1] * sub_type_g1_xyz[1] + sub_type_g1_xyz[2] * sub_type_g1_xyz[2] + self[e321] * self[e321]
                - sub_type_g0_xyz[0] * sub_type_g0_xyz[0]
                - sub_type_g0_xyz[1] * sub_type_g0_xyz[1]
                - sub_type_g0_xyz[2] * sub_type_g0_xyz[2]
                - self[e12345] * self[e12345],
        )
    }
}
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for VersorOdd {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        7        8        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g0_xyz = self.group1().xyz();
        let sub_type_g1_xyz = self.group3().xyz();
        Scalar::from_groups(
            // scalar
            sub_type_g0_xyz[0] * sub_type_g0_xyz[0] + sub_type_g0_xyz[1] * sub_type_g0_xyz[1] + sub_type_g0_xyz[2] * sub_type_g0_xyz[2] + self[scalar] * self[scalar]
                - sub_type_g1_xyz[0] * sub_type_g1_xyz[0]
                - sub_type_g1_xyz[1] * sub_type_g1_xyz[1]
                - sub_type_g1_xyz[2] * sub_type_g1_xyz[2]
                - self[e45] * self[e45],
        )
    }
}
