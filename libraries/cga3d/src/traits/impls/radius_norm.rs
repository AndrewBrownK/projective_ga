// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 25
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         3       4       0     N/A
//  Average:         4       7       0     N/A
//  Maximum:        23      32       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         3       4       0       0
//  Average:         4       7       0       0
//  Maximum:        23      32       0       0
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        7       11        0        0
    fn radius_norm(self) -> Scalar {
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
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32       10       15        0        0
    fn radius_norm(self) -> Scalar {
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
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiDualNum {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiDualNum {
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar])
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiFlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiFlatPoint {
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321])
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiFlector {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiLine {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiMotor {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar])
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiPlane {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiScalar {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e12345] * -1.0)
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for Circle {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        6       10        0        0
    fn radius_norm(self) -> Scalar {
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
impl std::ops::Div<RadiusNormPrefixOrPostfix> for CircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        7       11        0        0
    fn radius_norm(self) -> Scalar {
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
impl std::ops::Div<RadiusNormPrefixOrPostfix> for Dipole {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        6       10        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15]) + 2.0 * (self[e42] * self[e25]) + 2.0 * (self[e43] * self[e35]) + self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12]
                - self[e45] * self[e45],
        )
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for DipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32       10       15        0        0
    fn radius_norm(self) -> Scalar {
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
impl std::ops::Div<RadiusNormPrefixOrPostfix> for DualNum {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e12345] * -1.0)
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for FlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e45] * -1.0)
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for Flector {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -self[e45] * self[e45] - self[e4235] * self[e4235] - self[e4315] * self[e4315] - self[e4125] * self[e4125],
        )
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for Line {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435])
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for Motor {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435] - self[e12345] * self[e12345],
        )
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for MultiVector {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32       23       32        0        0
    fn radius_norm(self) -> Scalar {
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
impl std::ops::Div<RadiusNormPrefixOrPostfix> for Plane {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e4235] * self[e4235] - self[e4315] * self[e4315] - self[e4125] * self[e4125])
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for RoundPoint {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        5        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3] - 2.0 * (self[e4] * self[e5]))
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl std::ops::DivAssign<RadiusNormPrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: RadiusNormPrefixOrPostfix) {
        *self = self.radius_norm()
    }
}
impl RadiusNorm for Scalar {
    fn radius_norm(self) -> Scalar {
        self
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for Sphere {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        5        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            2.0 * (self[e3215] * self[e1234]) - self[e4235] * self[e4235] - self[e4315] * self[e4315] - self[e4125] * self[e4125],
        )
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for VersorEven {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32       11       16        0        0
    fn radius_norm(self) -> Scalar {
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
impl std::ops::Div<RadiusNormPrefixOrPostfix> for VersorOdd {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32       11       16        0        0
    fn radius_norm(self) -> Scalar {
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
