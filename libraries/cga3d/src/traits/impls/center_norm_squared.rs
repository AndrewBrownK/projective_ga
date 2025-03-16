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
//  Minimum:         3       0       0
//   Median:         6       0       0
//  Average:         6       0       0
//  Maximum:        15       0       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         3       0       0
//   Median:         6       0       0
//  Average:         6       0       0
//  Maximum:        15       0       0
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for AntiCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar] - self[e45] * self[e45],
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
    //      add/sub      mul      div
    // f32        6        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3] - self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435],
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
    //      add/sub      mul      div
    // f32        3        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321] - self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435])
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
    //      add/sub      mul      div
    // f32        4        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[e321] * self[e321] - self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435] - self[e12345] * self[e12345],
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
    //      add/sub      mul      div
    // f32        3        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] - self[e45] * self[e45])
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
    //      add/sub      mul      div
    // f32        6        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12]
                - self[e45] * self[e45]
                - self[e4235] * self[e4235]
                - self[e4315] * self[e4315]
                - self[e4125] * self[e4125],
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
    //      add/sub      mul      div
    // f32       15        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[scalar] * self[scalar]
                + self[e1] * self[e1]
                + self[e2] * self[e2]
                + self[e3] * self[e3]
                + self[e23] * self[e23]
                + self[e31] * self[e31]
                + self[e12] * self[e12]
                + self[e321] * self[e321]
                - self[e12345] * self[e12345]
                - self[e45] * self[e45]
                - self[e415] * self[e415]
                - self[e425] * self[e425]
                - self[e435] * self[e435]
                - self[e4235] * self[e4235]
                - self[e4315] * self[e4315]
                - self[e4125] * self[e4125],
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
    //      add/sub      mul      div
    // f32        7        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3]
                - self[e12345] * self[e12345]
                - self[e415] * self[e415]
                - self[e425] * self[e425]
                - self[e435] * self[e435],
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
    //      add/sub      mul      div
    // f32        7        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[scalar] * self[scalar] + self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12]
                - self[e45] * self[e45]
                - self[e4235] * self[e4235]
                - self[e4315] * self[e4315]
                - self[e4125] * self[e4125],
        )
    }
}
