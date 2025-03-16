// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 25
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       1       0
//  Average:         4       3       0
//  Maximum:        23      16       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       1       0
//  Average:         4       3       0
//  Maximum:        23      16       0
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for AntiCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        6        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + self[e23] * self[e23]
                + self[e31] * self[e31]
                + self[e12] * self[e12]
                + self[scalar] * self[scalar]
                - self[e45] * self[e45],
        )
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for AntiDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        8        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3]
                - self[e415] * self[e415]
                - self[e425] * self[e425]
                - self[e435] * self[e435]
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125])
                - 2.0 * (self[e4] * self[e5]),
        )
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for AntiDualNum {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiDualNum {
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * self[scalar])
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for AntiFlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiFlatPoint {
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321])
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for AntiFlector {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for AntiLine {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for AntiMotor {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar])
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for AntiPlane {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for AntiScalar {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e12345] * self[e12345] * -1.0)
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for Circle {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        6        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[e321] * self[e321]
                - self[e415] * self[e415]
                - self[e425] * self[e425]
                - self[e435] * self[e435]
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        )
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for CircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        6        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[e321] * self[e321]
                - self[e415] * self[e415]
                - self[e425] * self[e425]
                - self[e435] * self[e435]
                - self[e12345] * self[e12345]
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        )
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for Dipole {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        6        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15]) + 2.0 * (self[e42] * self[e25]) + 2.0 * (self[e43] * self[e35]) + self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12]
                - self[e45] * self[e45],
        )
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for DipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        8        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + 2.0 * (self[e1234] * self[e3215])
                + self[e23] * self[e23]
                + self[e31] * self[e31]
                + self[e12] * self[e12]
                - self[e45] * self[e45]
                - self[e4235] * self[e4235]
                - self[e4315] * self[e4315]
                - self[e4125] * self[e4125],
        )
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for DualNum {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e12345] * self[e12345] * -1.0)
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for FlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e45] * self[e45] * -1.0)
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for Flector {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -self[e45] * self[e45] - self[e4235] * self[e4235] - self[e4315] * self[e4315] - self[e4125] * self[e4125],
        )
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for Line {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435])
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for Motor {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435] - self[e12345] * self[e12345],
        )
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for MultiVector {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       23       16        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            2.0 * (self[e15] * self[e41])
                + 2.0 * (self[e25] * self[e42])
                + 2.0 * (self[e35] * self[e43])
                + 2.0 * (self[e3215] * self[e1234])
                + self[scalar] * self[scalar]
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
                - self[e4125] * self[e4125]
                - 2.0 * (self[e4] * self[e5])
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        )
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for Plane {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e4235] * self[e4235] - self[e4315] * self[e4315] - self[e4125] * self[e4125])
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for RoundPoint {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        2        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3] - 2.0 * (self[e4] * self[e5]))
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl std::ops::DivAssign<RadiusNormSquaredPrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: RadiusNormSquaredPrefixOrPostfix) {
        *self = self.radius_norm_squared()
    }
}
impl RadiusNormSquared for Scalar {
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * self[scalar])
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for Sphere {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        2        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            2.0 * (self[e3215] * self[e1234]) - self[e4235] * self[e4235] - self[e4315] * self[e4315] - self[e4125] * self[e4125],
        )
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for VersorEven {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        8        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3]
                - self[e12345] * self[e12345]
                - self[e415] * self[e415]
                - self[e425] * self[e425]
                - self[e435] * self[e435]
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125])
                - 2.0 * (self[e5] * self[e4]),
        )
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for VersorOdd {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        8        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + 2.0 * (self[e1234] * self[e3215])
                + self[scalar] * self[scalar]
                + self[e23] * self[e23]
                + self[e31] * self[e31]
                + self[e12] * self[e12]
                - self[e45] * self[e45]
                - self[e4235] * self[e4235]
                - self[e4315] * self[e4315]
                - self[e4125] * self[e4125],
        )
    }
}
