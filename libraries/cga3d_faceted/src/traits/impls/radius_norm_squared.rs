// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
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
impl RadiusNormSquared for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
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
impl RadiusNormSquared for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        6        0
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
                + self[scalar] * self[scalar],
        )
    }
}
impl RadiusNormSquared for AntiCircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar])
    }
}
impl RadiusNormSquared for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar] - self[e45] * self[e45],
        )
    }
}
impl RadiusNormSquared for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * self[scalar] + self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
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
impl RadiusNormSquared for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3] - self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435],
        )
    }
}
impl RadiusNormSquared for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RadiusNormSquared for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        8        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -self[e415] * self[e415]
                - self[e425] * self[e425]
                - self[e435] * self[e435]
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125])
                - 2.0 * (self[e5] * self[e4]),
        )
    }
}
impl RadiusNormSquared for AntiDipoleOnOrigin {
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321])
    }
}
impl RadiusNormSquared for AntiDualNum {
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * self[scalar])
    }
}
impl RadiusNormSquared for AntiFlatOrigin {
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321])
    }
}
impl RadiusNormSquared for AntiFlatPoint {
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321])
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
impl RadiusNormSquared for AntiFlectorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
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
impl RadiusNormSquared for AntiLineOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
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
impl RadiusNormSquared for AntiMotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar])
    }
}
impl RadiusNormSquared for AntiMysteryCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar] - self[e45] * self[e45],
        )
    }
}
impl RadiusNormSquared for AntiMysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3] - self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435],
        )
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
impl RadiusNormSquared for AntiPlaneOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
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
impl RadiusNormSquared for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RadiusNormSquared for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * self[scalar] + self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
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
impl RadiusNormSquared for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -self[e415] * self[e415]
                - self[e425] * self[e425]
                - self[e435] * self[e435]
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        )
    }
}
impl RadiusNormSquared for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321] - self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435])
    }
}
impl RadiusNormSquared for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -2.0 * (self[e423] * self[e235]) - 2.0 * (self[e431] * self[e315]) - 2.0 * (self[e412] * self[e125]))
    }
}
impl RadiusNormSquared for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435])
    }
}
impl RadiusNormSquared for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[e321] * self[e321] - 2.0 * (self[e423] * self[e235]) - 2.0 * (self[e431] * self[e315]) - 2.0 * (self[e412] * self[e125]),
        )
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
impl RadiusNormSquared for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        6        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -self[e415] * self[e415]
                - self[e425] * self[e425]
                - self[e435] * self[e435]
                - self[e12345] * self[e12345]
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        )
    }
}
impl RadiusNormSquared for CircleRotorAligningOriginAtInfinity {
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
impl RadiusNormSquared for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[e321] * self[e321] - self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435] - self[e12345] * self[e12345],
        )
    }
}
impl RadiusNormSquared for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -self[e12345] * self[e12345] - self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435],
        )
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
impl RadiusNormSquared for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15]) + 2.0 * (self[e42] * self[e25]) + 2.0 * (self[e43] * self[e35]) - self[e45] * self[e45],
        )
    }
}
impl RadiusNormSquared for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] - self[e45] * self[e45])
    }
}
impl RadiusNormSquared for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ 2.0 * (self[e41] * self[e15]) + 2.0 * (self[e42] * self[e25]) + 2.0 * (self[e43] * self[e35]))
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
impl RadiusNormSquared for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15]) + 2.0 * (self[e42] * self[e25]) + 2.0 * (self[e43] * self[e35]) + 2.0 * (self[e1234] * self[e3215])
                - self[e45] * self[e45]
                - self[e4235] * self[e4235]
                - self[e4315] * self[e4315]
                - self[e4125] * self[e4125],
        )
    }
}
impl RadiusNormSquared for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn radius_norm_squared(self) -> Scalar {
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
impl RadiusNormSquared for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15]) + 2.0 * (self[e42] * self[e25]) + 2.0 * (self[e43] * self[e35]) + 2.0 * (self[e3215] * self[e1234]),
        )
    }
}
impl RadiusNormSquared for DipoleInversionOnOrigin {
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
impl RadiusNormSquared for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        8        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + 2.0 * (self[e3215] * self[e1234])
                + self[e23] * self[e23]
                + self[e31] * self[e31]
                + self[e12] * self[e12],
        )
    }
}
impl RadiusNormSquared for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e45] * self[e45] * -1.0)
    }
}
impl RadiusNormSquared for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15]) + 2.0 * (self[e42] * self[e25]) + 2.0 * (self[e43] * self[e35]) + self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12],
        )
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
impl RadiusNormSquared for FlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e45] * self[e45] * -1.0)
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
impl RadiusNormSquared for FlectorOnOrigin {
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
impl RadiusNormSquared for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435])
    }
}
impl RadiusNormSquared for LineOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435])
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
impl RadiusNormSquared for MotorOnOrigin {
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
impl RadiusNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       23       16        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + 2.0 * (self[e1234] * self[e3215])
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
impl RadiusNormSquared for MysteryCircle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321] - self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435])
    }
}
impl RadiusNormSquared for MysteryCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[e321] * self[e321] - self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435] - self[e12345] * self[e12345],
        )
    }
}
impl RadiusNormSquared for MysteryDipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] - self[e45] * self[e45])
    }
}
impl RadiusNormSquared for MysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn radius_norm_squared(self) -> Scalar {
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
impl RadiusNormSquared for MysteryVersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3] + self[e321] * self[e321]
                - self[e12345] * self[e12345]
                - self[e415] * self[e415]
                - self[e425] * self[e425]
                - self[e435] * self[e435],
        )
    }
}
impl RadiusNormSquared for MysteryVersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[scalar] * self[scalar] + self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12]
                - self[e4235] * self[e4235]
                - self[e4315] * self[e4315]
                - self[e4125] * self[e4125]
                - self[e45] * self[e45],
        )
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
impl RadiusNormSquared for PlaneOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e4235] * self[e4235] - self[e4315] * self[e4315] - self[e4125] * self[e4125])
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
impl RadiusNormSquared for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e4] * self[e5] * -2.0)
    }
}
impl RadiusNormSquared for Scalar {
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * self[scalar])
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
impl RadiusNormSquared for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e3215] * self[e1234] * 2.0)
    }
}
impl RadiusNormSquared for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e4235] * self[e4235] - self[e4315] * self[e4315] - self[e4125] * self[e4125])
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
impl RadiusNormSquared for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -self[e12345] * self[e12345]
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
impl RadiusNormSquared for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3] + self[e321] * self[e321]
                - self[e12345] * self[e12345]
                - self[e415] * self[e415]
                - self[e425] * self[e425]
                - self[e435] * self[e435],
        )
    }
}
impl RadiusNormSquared for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -2.0 * (self[e423] * self[e235]) - 2.0 * (self[e431] * self[e315]) - 2.0 * (self[e412] * self[e125]) - 2.0 * (self[e4] * self[e5]),
        )
    }
}
impl RadiusNormSquared for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -self[e12345] * self[e12345] - self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435],
        )
    }
}
impl RadiusNormSquared for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3]
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125])
                - 2.0 * (self[e5] * self[e4]),
        )
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
impl RadiusNormSquared for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn radius_norm_squared(self) -> Scalar {
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
impl RadiusNormSquared for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn radius_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + 2.0 * (self[e3215] * self[e1234])
                + self[scalar] * self[scalar]
                + self[e23] * self[e23]
                + self[e31] * self[e31]
                + self[e12] * self[e12],
        )
    }
}
