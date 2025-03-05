// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 23
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         5       0       0
//  Average:         5       1       0
//  Maximum:        46      41       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         5       0       0
//  Average:         5       2       0
//  Maximum:        46      49       0
impl std::ops::Div<RoundNormSquaredPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let sub_type = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group1());
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2),
                f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<RoundNormSquaredPrefixOrPostfix> for AntiCircleRotor {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let sub_type = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]));
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2) + f32::powi(sub_type[scalar], 2),
                f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<RoundNormSquaredPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let sub_type = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ self.group1().with_w(self[scalar]));
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2) + f32::powi(sub_type[scalar], 2),
                f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<RoundNormSquaredPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let sub_type = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ self.group1().with_w(self[scalar]));
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2) + f32::powi(sub_type[scalar], 2),
                f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<RoundNormSquaredPrefixOrPostfix> for AntiDipoleInversion {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let sub_type = AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]));
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0().with_w(self[e4]).wxyz());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                f32::powi(sub_type[e321], 2) + f32::powi(sub_type[e1], 2) + f32::powi(sub_type[e2], 2) + f32::powi(sub_type[e3], 2),
                f32::powi(wedge[e45], 2) + f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<RoundNormSquaredPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let sub_type = AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]));
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                f32::powi(sub_type[e321], 2) + f32::powi(sub_type[e1], 2) + f32::powi(sub_type[e2], 2) + f32::powi(sub_type[e3], 2),
                f32::powi(wedge[e45], 2) + f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<RoundNormSquaredPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([f32::powi(self[e321], 2), f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<RoundNormSquaredPrefixOrPostfix> for AntiDualNum {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for AntiDualNum {
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([f32::powi(self[scalar], 2), f32::powi(self[e1234], 2)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<RoundNormSquaredPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let sub_type = AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0().xyz());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([f32::powi(sub_type[e1], 2) + f32::powi(sub_type[e2], 2) + f32::powi(sub_type[e3], 2), f32::powi(self[e4], 2)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<RoundNormSquaredPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let sub_type = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]));
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2) + f32::powi(sub_type[scalar], 2),
                f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2) + f32::powi(wedge[e12345], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<RoundNormSquaredPrefixOrPostfix> for Circle {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([f32::powi(self[e321], 2), f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<RoundNormSquaredPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([f32::powi(self[e321], 2), f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<RoundNormSquaredPrefixOrPostfix> for CircleRotor {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([f32::powi(self[e321], 2), f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<RoundNormSquaredPrefixOrPostfix> for Dipole {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let sub_type = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group1().xyz());
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2),
                f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<RoundNormSquaredPrefixOrPostfix> for DipoleInversion {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let sub_type = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group1().xyz());
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0().with_w(self[e1234]));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2),
                f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2) + f32::powi(wedge[e12345], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<RoundNormSquaredPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let sub_type = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group1());
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2),
                f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2) + f32::powi(wedge[e12345], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<RoundNormSquaredPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let sub_type = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group1());
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2),
                f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<RoundNormSquaredPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_norm_squared()
    }
}
impl std::ops::DivAssign<RoundNormSquaredPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: RoundNormSquaredPrefixOrPostfix) {
        *self = self.round_norm_squared()
    }
}
impl RoundNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       46       37        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       46       41        0
    //  no simd       46       49        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let sub_type = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        let sub_type_2 = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group3().xyz().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
        let other = Infinity::from_groups(/* e5 */ 1.0);
        let wedge = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e5] * sub_type_2[e1234]]) * Simd32x2::from([0.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e5] * sub_type_2[e4]),
            // e15, e25, e35
            Simd32x3::from(other[e5]) * sub_type_2.group1().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (Simd32x3::from(other[e5]) * sub_type_2.group3().xyz()).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e5] * sub_type_2[e423], other[e5] * sub_type_2[e431], other[e5] * sub_type_2[e412]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]),
            // e3215
            0.0,
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                2.0 * (sub_type[e41] * sub_type[e15])
                    + 2.0 * (sub_type[e42] * sub_type[e25])
                    + 2.0 * (sub_type[e43] * sub_type[e35])
                    + 2.0 * (sub_type[e1234] * sub_type[e3215])
                    + f32::powi(sub_type[scalar], 2)
                    + f32::powi(sub_type[e1], 2)
                    + f32::powi(sub_type[e2], 2)
                    + f32::powi(sub_type[e3], 2)
                    + f32::powi(sub_type[e23], 2)
                    + f32::powi(sub_type[e31], 2)
                    + f32::powi(sub_type[e12], 2)
                    + f32::powi(sub_type[e321], 2)
                    - f32::powi(sub_type[e12345], 2)
                    - f32::powi(sub_type[e45], 2)
                    - f32::powi(sub_type[e415], 2)
                    - f32::powi(sub_type[e425], 2)
                    - f32::powi(sub_type[e435], 2)
                    - f32::powi(sub_type[e4235], 2)
                    - f32::powi(sub_type[e4315], 2)
                    - f32::powi(sub_type[e4125], 2)
                    - 2.0 * (sub_type[e4] * sub_type[e5])
                    - 2.0 * (sub_type[e423] * sub_type[e235])
                    - 2.0 * (sub_type[e431] * sub_type[e315])
                    - 2.0 * (sub_type[e412] * sub_type[e125]),
                2.0 * (wedge[e4] * wedge[e5])
                    + 2.0 * (wedge[e423] * wedge[e235])
                    + 2.0 * (wedge[e431] * wedge[e315])
                    + 2.0 * (wedge[e412] * wedge[e125])
                    + f32::powi(wedge[e12345], 2)
                    + f32::powi(wedge[e45], 2)
                    + f32::powi(wedge[e415], 2)
                    + f32::powi(wedge[e425], 2)
                    + f32::powi(wedge[e435], 2)
                    + f32::powi(wedge[e4235], 2)
                    + f32::powi(wedge[e4315], 2)
                    + f32::powi(wedge[e4125], 2)
                    - f32::powi(wedge[scalar], 2)
                    - f32::powi(wedge[e1], 2)
                    - f32::powi(wedge[e2], 2)
                    - f32::powi(wedge[e3], 2)
                    - f32::powi(wedge[e23], 2)
                    - f32::powi(wedge[e31], 2)
                    - f32::powi(wedge[e12], 2)
                    - f32::powi(wedge[e321], 2)
                    - 2.0 * (wedge[e41] * wedge[e15])
                    - 2.0 * (wedge[e42] * wedge[e25])
                    - 2.0 * (wedge[e43] * wedge[e35])
                    - 2.0 * (wedge[e1234] * wedge[e3215]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<RoundNormSquaredPrefixOrPostfix> for RoundPoint {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let sub_type = AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0().xyz());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([f32::powi(sub_type[e1], 2) + f32::powi(sub_type[e2], 2) + f32::powi(sub_type[e3], 2), f32::powi(self[e4], 2)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<RoundNormSquaredPrefixOrPostfix> for VersorEven {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let sub_type = AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]));
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                f32::powi(sub_type[e321], 2) + f32::powi(sub_type[e1], 2) + f32::powi(sub_type[e2], 2) + f32::powi(sub_type[e3], 2),
                f32::powi(wedge[e45], 2) + f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<RoundNormSquaredPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let sub_type = AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]));
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                f32::powi(sub_type[e321], 2) + f32::powi(sub_type[e1], 2) + f32::powi(sub_type[e2], 2) + f32::powi(sub_type[e3], 2),
                f32::powi(wedge[e45], 2) + f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<RoundNormSquaredPrefixOrPostfix> for VersorOdd {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let sub_type = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]));
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2) + f32::powi(sub_type[scalar], 2),
                f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2) + f32::powi(wedge[e12345], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<RoundNormSquaredPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let sub_type = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]));
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2) + f32::powi(sub_type[scalar], 2),
                f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2) + f32::powi(wedge[e12345], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
