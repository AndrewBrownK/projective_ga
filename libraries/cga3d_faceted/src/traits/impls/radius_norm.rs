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
impl RadiusNorm for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RadiusNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        6        0
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
impl RadiusNorm for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        6        0
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
                + self[scalar] * self[scalar],
        )
    }
}
impl RadiusNorm for AntiCircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar])
    }
}
impl RadiusNorm for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar] - self[e45] * self[e45],
        )
    }
}
impl RadiusNorm for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * self[scalar] + self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RadiusNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        8        0
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
impl RadiusNorm for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3] - self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435],
        )
    }
}
impl RadiusNorm for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RadiusNorm for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        8        0
    fn radius_norm(self) -> Scalar {
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
impl RadiusNorm for AntiDipoleOnOrigin {
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321])
    }
}
impl RadiusNorm for AntiDualNum {
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar])
    }
}
impl RadiusNorm for AntiFlatOrigin {
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321])
    }
}
impl RadiusNorm for AntiFlatPoint {
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321])
    }
}
impl RadiusNorm for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RadiusNorm for AntiFlectorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RadiusNorm for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RadiusNorm for AntiLineOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RadiusNorm for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar])
    }
}
impl RadiusNorm for AntiMotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar])
    }
}
impl RadiusNorm for AntiMysteryCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar] - self[e45] * self[e45],
        )
    }
}
impl RadiusNorm for AntiMysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3] - self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435],
        )
    }
}
impl RadiusNorm for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RadiusNorm for AntiPlaneOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RadiusNorm for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e12345] * -1.0)
    }
}
impl RadiusNorm for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RadiusNorm for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * self[scalar] + self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RadiusNorm for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        6        0
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
impl RadiusNorm for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn radius_norm(self) -> Scalar {
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
impl RadiusNorm for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321] - self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435])
    }
}
impl RadiusNorm for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -2.0 * (self[e423] * self[e235]) - 2.0 * (self[e431] * self[e315]) - 2.0 * (self[e412] * self[e125]))
    }
}
impl RadiusNorm for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435])
    }
}
impl RadiusNorm for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[e321] * self[e321] - 2.0 * (self[e423] * self[e235]) - 2.0 * (self[e431] * self[e315]) - 2.0 * (self[e412] * self[e125]),
        )
    }
}
impl RadiusNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        6        0
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
impl RadiusNorm for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        6        0
    fn radius_norm(self) -> Scalar {
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
impl RadiusNorm for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435] - self[e12345] * self[e12345],
        )
    }
}
impl RadiusNorm for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[e321] * self[e321] - self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435] - self[e12345] * self[e12345],
        )
    }
}
impl RadiusNorm for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -self[e12345] * self[e12345] - self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435],
        )
    }
}
impl RadiusNorm for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        6        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15]) + 2.0 * (self[e42] * self[e25]) + 2.0 * (self[e43] * self[e35]) + self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12]
                - self[e45] * self[e45],
        )
    }
}
impl RadiusNorm for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15]) + 2.0 * (self[e42] * self[e25]) + 2.0 * (self[e43] * self[e35]) - self[e45] * self[e45],
        )
    }
}
impl RadiusNorm for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] - self[e45] * self[e45])
    }
}
impl RadiusNorm for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ 2.0 * (self[e41] * self[e15]) + 2.0 * (self[e42] * self[e25]) + 2.0 * (self[e43] * self[e35]))
    }
}
impl RadiusNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        8        0
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
impl RadiusNorm for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn radius_norm(self) -> Scalar {
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
impl RadiusNorm for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn radius_norm(self) -> Scalar {
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
impl RadiusNorm for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15]) + 2.0 * (self[e42] * self[e25]) + 2.0 * (self[e43] * self[e35]) + 2.0 * (self[e3215] * self[e1234]),
        )
    }
}
impl RadiusNorm for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -self[e45] * self[e45] - self[e4235] * self[e4235] - self[e4315] * self[e4315] - self[e4125] * self[e4125],
        )
    }
}
impl RadiusNorm for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        8        0
    fn radius_norm(self) -> Scalar {
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
impl RadiusNorm for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e45] * -1.0)
    }
}
impl RadiusNorm for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15]) + 2.0 * (self[e42] * self[e25]) + 2.0 * (self[e43] * self[e35]) + self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12],
        )
    }
}
impl RadiusNorm for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e12345] * -1.0)
    }
}
impl RadiusNorm for FlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e45] * -1.0)
    }
}
impl RadiusNorm for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e45] * -1.0)
    }
}
impl RadiusNorm for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -self[e45] * self[e45] - self[e4235] * self[e4235] - self[e4315] * self[e4315] - self[e4125] * self[e4125],
        )
    }
}
impl RadiusNorm for FlectorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -self[e45] * self[e45] - self[e4235] * self[e4235] - self[e4315] * self[e4315] - self[e4125] * self[e4125],
        )
    }
}
impl RadiusNorm for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435])
    }
}
impl RadiusNorm for LineOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435])
    }
}
impl RadiusNorm for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435] - self[e12345] * self[e12345],
        )
    }
}
impl RadiusNorm for MotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435] - self[e12345] * self[e12345],
        )
    }
}
impl RadiusNorm for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       23       16        0
    fn radius_norm(self) -> Scalar {
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
impl RadiusNorm for MysteryCircle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321] - self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435])
    }
}
impl RadiusNorm for MysteryCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[e321] * self[e321] - self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435] - self[e12345] * self[e12345],
        )
    }
}
impl RadiusNorm for MysteryDipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] - self[e45] * self[e45])
    }
}
impl RadiusNorm for MysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn radius_norm(self) -> Scalar {
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
impl RadiusNorm for MysteryVersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn radius_norm(self) -> Scalar {
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
impl RadiusNorm for MysteryVersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn radius_norm(self) -> Scalar {
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
impl RadiusNorm for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e4235] * self[e4235] - self[e4315] * self[e4315] - self[e4125] * self[e4125])
    }
}
impl RadiusNorm for PlaneOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e4235] * self[e4235] - self[e4315] * self[e4315] - self[e4125] * self[e4125])
    }
}
impl RadiusNorm for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        2        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3] - 2.0 * (self[e4] * self[e5]))
    }
}
impl RadiusNorm for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ f32::powf(self[e4], 0.5) * f32::powf(self[e5], 0.5) * -2.0)
    }
}
impl RadiusNorm for Scalar {
    fn radius_norm(self) -> Scalar {
        self
    }
}
impl RadiusNorm for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        2        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            2.0 * (self[e3215] * self[e1234]) - self[e4235] * self[e4235] - self[e4315] * self[e4315] - self[e4125] * self[e4125],
        )
    }
}
impl RadiusNorm for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ f32::powf(self[e3215], 0.5) * f32::powf(self[e1234], 0.5) * 2.0)
    }
}
impl RadiusNorm for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e4235] * self[e4235] - self[e4315] * self[e4315] - self[e4125] * self[e4125])
    }
}
impl RadiusNorm for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        8        0
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
impl RadiusNorm for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn radius_norm(self) -> Scalar {
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
impl RadiusNorm for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn radius_norm(self) -> Scalar {
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
impl RadiusNorm for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -2.0 * (self[e423] * self[e235]) - 2.0 * (self[e431] * self[e315]) - 2.0 * (self[e412] * self[e125]) - 2.0 * (self[e4] * self[e5]),
        )
    }
}
impl RadiusNorm for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn radius_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -self[e12345] * self[e12345] - self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435],
        )
    }
}
impl RadiusNorm for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn radius_norm(self) -> Scalar {
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
impl RadiusNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        8        0
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
impl RadiusNorm for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn radius_norm(self) -> Scalar {
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
impl RadiusNorm for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn radius_norm(self) -> Scalar {
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
