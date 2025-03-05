// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 83
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       0       0
//  Average:         3       2       0
//  Maximum:        23      16       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       0       0
//  Average:         3       2       0
//  Maximum:        23      16       0
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2));
    }
}
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
        return Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                + f32::powi(self[scalar], 2)
                - f32::powi(self[e45], 2),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        6        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                + f32::powi(self[scalar], 2),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiCircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) + f32::powi(self[scalar], 2),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) + f32::powi(self[scalar], 2) - f32::powi(self[e45], 2),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            f32::powi(self[scalar], 2) + f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2),
        );
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
        return Scalar::from_groups(
            // scalar
            f32::powi(self[e321], 2) + f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125])
                - 2.0 * (self[e4] * self[e5]),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            f32::powi(self[e321], 2) + f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powi(self[e321], 2) + f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2));
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        8        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            -f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125])
                - 2.0 * (self[e5] * self[e4]),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiDipoleOnOrigin {
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powi(self[e321], 2));
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
        return Scalar::from_groups(/* scalar */ f32::powi(self[scalar], 2));
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for AntiFlatOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiFlatOrigin {
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powi(self[e321], 2));
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
        return Scalar::from_groups(/* scalar */ f32::powi(self[e321], 2));
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
        return Scalar::from_groups(/* scalar */ f32::powi(self[e321], 2) + f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2));
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for AntiFlectorOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiFlectorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powi(self[e321], 2) + f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2));
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
        return Scalar::from_groups(/* scalar */ f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2));
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for AntiLineOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiLineOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2));
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
        return Scalar::from_groups(
            // scalar
            f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) + f32::powi(self[scalar], 2),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for AntiMotorOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiMotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) + f32::powi(self[scalar], 2),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for AntiMysteryCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiMysteryCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) + f32::powi(self[scalar], 2) - f32::powi(self[e45], 2),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for AntiMysteryDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiMysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            f32::powi(self[e321], 2) + f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2),
        );
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
        return Scalar::from_groups(/* scalar */ f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2));
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for AntiPlaneOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiPlaneOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2));
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
        return Scalar::from_groups(/* scalar */ f32::powi(self[e12345], 2) * -1.0);
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2));
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            f32::powi(self[scalar], 2) + f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2),
        );
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
        return Scalar::from_groups(
            // scalar
            f32::powi(self[e321], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for CircleAligningOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            -f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for CircleAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            f32::powi(self[e321], 2) - f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for CircleAtOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -2.0 * (self[e423] * self[e235]) - 2.0 * (self[e431] * self[e315]) - 2.0 * (self[e412] * self[e125]));
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for CircleOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2));
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            f32::powi(self[e321], 2) - 2.0 * (self[e423] * self[e235]) - 2.0 * (self[e431] * self[e315]) - 2.0 * (self[e412] * self[e125]),
        );
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
        return Scalar::from_groups(
            // scalar
            f32::powi(self[e321], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - f32::powi(self[e12345], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        6        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            -f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - f32::powi(self[e12345], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            -f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2) - f32::powi(self[e12345], 2),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            f32::powi(self[e321], 2) - f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2) - f32::powi(self[e12345], 2),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            -f32::powi(self[e12345], 2) - f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2),
        );
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
        return Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                - f32::powi(self[e45], 2),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15]) + 2.0 * (self[e42] * self[e25]) + 2.0 * (self[e43] * self[e35]) - f32::powi(self[e45], 2),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for DipoleAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) - f32::powi(self[e45], 2));
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for DipoleAtOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ 2.0 * (self[e41] * self[e15]) + 2.0 * (self[e42] * self[e25]) + 2.0 * (self[e43] * self[e35]));
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
        return Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + 2.0 * (self[e1234] * self[e3215])
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                - f32::powi(self[e45], 2)
                - f32::powi(self[e4235], 2)
                - f32::powi(self[e4315], 2)
                - f32::powi(self[e4125], 2),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15]) + 2.0 * (self[e42] * self[e25]) + 2.0 * (self[e43] * self[e35]) + 2.0 * (self[e1234] * self[e3215])
                - f32::powi(self[e45], 2)
                - f32::powi(self[e4235], 2)
                - f32::powi(self[e4315], 2)
                - f32::powi(self[e4125], 2),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2)
                - f32::powi(self[e45], 2)
                - f32::powi(self[e4235], 2)
                - f32::powi(self[e4315], 2)
                - f32::powi(self[e4125], 2),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15]) + 2.0 * (self[e42] * self[e25]) + 2.0 * (self[e43] * self[e35]) + 2.0 * (self[e3215] * self[e1234]),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            -f32::powi(self[e45], 2) - f32::powi(self[e4235], 2) - f32::powi(self[e4315], 2) - f32::powi(self[e4125], 2),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        8        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + 2.0 * (self[e3215] * self[e1234])
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for DipoleOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powi(self[e45], 2) * -1.0);
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2),
        );
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
        return Scalar::from_groups(/* scalar */ f32::powi(self[e12345], 2) * -1.0);
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for FlatOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for FlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powi(self[e45], 2) * -1.0);
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
        return Scalar::from_groups(/* scalar */ f32::powi(self[e45], 2) * -1.0);
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
        return Scalar::from_groups(
            // scalar
            -f32::powi(self[e45], 2) - f32::powi(self[e4235], 2) - f32::powi(self[e4315], 2) - f32::powi(self[e4125], 2),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for FlectorOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for FlectorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            -f32::powi(self[e45], 2) - f32::powi(self[e4235], 2) - f32::powi(self[e4315], 2) - f32::powi(self[e4125], 2),
        );
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
        return Scalar::from_groups(/* scalar */ -f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2));
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for LineOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for LineOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2));
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
        return Scalar::from_groups(
            // scalar
            -f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2) - f32::powi(self[e12345], 2),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for MotorOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for MotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            -f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2) - f32::powi(self[e12345], 2),
        );
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
        return Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + 2.0 * (self[e1234] * self[e3215])
                + f32::powi(self[scalar], 2)
                + f32::powi(self[e1], 2)
                + f32::powi(self[e2], 2)
                + f32::powi(self[e3], 2)
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                + f32::powi(self[e321], 2)
                - f32::powi(self[e12345], 2)
                - f32::powi(self[e45], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - f32::powi(self[e4235], 2)
                - f32::powi(self[e4315], 2)
                - f32::powi(self[e4125], 2)
                - 2.0 * (self[e4] * self[e5])
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for MysteryCircle {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for MysteryCircle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            f32::powi(self[e321], 2) - f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for MysteryCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for MysteryCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            f32::powi(self[e321], 2) - f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2) - f32::powi(self[e12345], 2),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for MysteryDipole {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for MysteryDipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) - f32::powi(self[e45], 2));
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for MysteryDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for MysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2)
                - f32::powi(self[e45], 2)
                - f32::powi(self[e4235], 2)
                - f32::powi(self[e4315], 2)
                - f32::powi(self[e4125], 2),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for MysteryVersorEven {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for MysteryVersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2) + f32::powi(self[e321], 2)
                - f32::powi(self[e12345], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for MysteryVersorOdd {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for MysteryVersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            f32::powi(self[scalar], 2) + f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2)
                - f32::powi(self[e4235], 2)
                - f32::powi(self[e4315], 2)
                - f32::powi(self[e4125], 2)
                - f32::powi(self[e45], 2),
        );
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
        return Scalar::from_groups(/* scalar */ -f32::powi(self[e4235], 2) - f32::powi(self[e4315], 2) - f32::powi(self[e4125], 2));
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for PlaneOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for PlaneOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -f32::powi(self[e4235], 2) - f32::powi(self[e4315], 2) - f32::powi(self[e4125], 2));
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
        return Scalar::from_groups(/* scalar */ f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2) - 2.0 * (self[e4] * self[e5]));
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for RoundPointAtOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e4] * self[e5] * -2.0);
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
        return Scalar::from_groups(/* scalar */ f32::powi(self[scalar], 2));
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
        return Scalar::from_groups(
            // scalar
            2.0 * (self[e3215] * self[e1234]) - f32::powi(self[e4235], 2) - f32::powi(self[e4315], 2) - f32::powi(self[e4125], 2),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for SphereAtOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e3215] * self[e1234] * 2.0);
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for SphereOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -f32::powi(self[e4235], 2) - f32::powi(self[e4315], 2) - f32::powi(self[e4125], 2));
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
        return Scalar::from_groups(
            // scalar
            f32::powi(self[e321], 2) + f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2)
                - f32::powi(self[e12345], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125])
                - 2.0 * (self[e5] * self[e4]),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            -f32::powi(self[e12345], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125])
                - 2.0 * (self[e4] * self[e5]),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2) + f32::powi(self[e321], 2)
                - f32::powi(self[e12345], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            -2.0 * (self[e423] * self[e235]) - 2.0 * (self[e431] * self[e315]) - 2.0 * (self[e412] * self[e125]) - 2.0 * (self[e4] * self[e5]),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            -f32::powi(self[e12345], 2) - f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            f32::powi(self[e321], 2) + f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125])
                - 2.0 * (self[e5] * self[e4]),
        );
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
        return Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + 2.0 * (self[e1234] * self[e3215])
                + f32::powi(self[scalar], 2)
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                - f32::powi(self[e45], 2)
                - f32::powi(self[e4235], 2)
                - f32::powi(self[e4315], 2)
                - f32::powi(self[e4125], 2),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            f32::powi(self[scalar], 2) + f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2)
                - f32::powi(self[e45], 2)
                - f32::powi(self[e4235], 2)
                - f32::powi(self[e4315], 2)
                - f32::powi(self[e4125], 2),
        );
    }
}
impl std::ops::Div<RadiusNormSquaredPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + 2.0 * (self[e3215] * self[e1234])
                + f32::powi(self[scalar], 2)
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2),
        );
    }
}
