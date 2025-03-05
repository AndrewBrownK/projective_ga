// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 9
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         3       0       0
//   Median:         6       0       0
//  Average:         9       3       0
//  Maximum:        47      32       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         3       0       0
//   Median:         6       0       0
//  Average:         9       3       0
//  Maximum:        47      32       0
impl std::ops::Div<CenterNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(0.0),
        );
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2) + f32::powi(sub_type[scalar], 2)
                - f32::powi(Simd32x3::from(0.0).with_w(self[e45])[3], 2),
        );
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for AntiDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e1, e2, e3, e5
            self.group3().xyz().with_w(0.0),
        );
        let sub_type_2 = Line::from_groups(/* e415, e425, e435 */ self.group1().xyz(), /* e235, e315, e125 */ Simd32x3::from(0.0));
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e321], 2) + f32::powi(sub_type[e1], 2) + f32::powi(sub_type[e2], 2) + f32::powi(sub_type[e3], 2)
                - f32::powi(sub_type_2[e415], 2)
                - f32::powi(sub_type_2[e425], 2)
                - f32::powi(sub_type_2[e435], 2),
        );
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for Circle {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type_2 = Line::from_groups(/* e415, e425, e435 */ self.group1().xyz(), /* e235, e315, e125 */ Simd32x3::from(0.0));
        return Scalar::from_groups(
            // scalar
            f32::powi(Simd32x3::from(0.0).with_w(self[e321])[3], 2) - f32::powi(sub_type_2[e415], 2) - f32::powi(sub_type_2[e425], 2) - f32::powi(sub_type_2[e435], 2),
        );
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for CircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type_2 = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
        return Scalar::from_groups(
            // scalar
            f32::powi(Simd32x3::from(0.0).with_w(self[e321])[3], 2)
                - f32::powi(sub_type_2[e415], 2)
                - f32::powi(sub_type_2[e425], 2)
                - f32::powi(sub_type_2[e435], 2)
                - f32::powi(sub_type_2[e12345], 2),
        );
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for Dipole {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiLine::from_groups(/* e23, e31, e12 */ self.group1().xyz(), /* e15, e25, e35 */ Simd32x3::from(0.0));
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2) - f32::powi(Simd32x3::from(0.0).with_w(self[e45])[3], 2),
        );
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for DipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiLine::from_groups(/* e23, e31, e12 */ self.group1().xyz(), /* e15, e25, e35 */ Simd32x3::from(0.0));
        let sub_type_2 = Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(self[e45]),
            // e4235, e4315, e4125, e3215
            self.group3().xyz().with_w(0.0),
        );
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2)
                - f32::powi(sub_type_2[e45], 2)
                - f32::powi(sub_type_2[e4235], 2)
                - f32::powi(sub_type_2[e4315], 2)
                - f32::powi(sub_type_2[e4125], 2),
        );
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for MultiVector {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       47       32        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
        let sub_type_2 = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(self[e45]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group6().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            self.group9().xyz().with_w(0.0),
            // e1234
            0.0,
        );
        return Scalar::from_groups(
            // scalar
            2.0 * (sub_type[e15] * sub_type[e41])
                + 2.0 * (sub_type[e25] * sub_type[e42])
                + 2.0 * (sub_type[e35] * sub_type[e43])
                + 2.0 * (sub_type[e3215] * sub_type[e1234])
                + 2.0 * (sub_type_2[e15] * sub_type_2[e41])
                + 2.0 * (sub_type_2[e25] * sub_type_2[e42])
                + 2.0 * (sub_type_2[e35] * sub_type_2[e43])
                + 2.0 * (sub_type_2[e3215] * sub_type_2[e1234])
                + f32::powi(sub_type[scalar], 2)
                + f32::powi(sub_type[e1], 2)
                + f32::powi(sub_type[e2], 2)
                + f32::powi(sub_type[e3], 2)
                + f32::powi(sub_type[e23], 2)
                + f32::powi(sub_type[e31], 2)
                + f32::powi(sub_type[e12], 2)
                + f32::powi(sub_type[e321], 2)
                + f32::powi(sub_type_2[scalar], 2)
                + f32::powi(sub_type_2[e1], 2)
                + f32::powi(sub_type_2[e2], 2)
                + f32::powi(sub_type_2[e3], 2)
                + f32::powi(sub_type_2[e23], 2)
                + f32::powi(sub_type_2[e31], 2)
                + f32::powi(sub_type_2[e12], 2)
                + f32::powi(sub_type_2[e321], 2)
                - f32::powi(sub_type[e12345], 2)
                - f32::powi(sub_type[e45], 2)
                - f32::powi(sub_type[e415], 2)
                - f32::powi(sub_type[e425], 2)
                - f32::powi(sub_type[e435], 2)
                - f32::powi(sub_type[e4235], 2)
                - f32::powi(sub_type[e4315], 2)
                - f32::powi(sub_type[e4125], 2)
                - f32::powi(sub_type_2[e12345], 2)
                - f32::powi(sub_type_2[e45], 2)
                - f32::powi(sub_type_2[e415], 2)
                - f32::powi(sub_type_2[e425], 2)
                - f32::powi(sub_type_2[e435], 2)
                - f32::powi(sub_type_2[e4235], 2)
                - f32::powi(sub_type_2[e4315], 2)
                - f32::powi(sub_type_2[e4125], 2)
                - 2.0 * (sub_type[e4] * sub_type[e5])
                - 2.0 * (sub_type[e423] * sub_type[e235])
                - 2.0 * (sub_type[e431] * sub_type[e315])
                - 2.0 * (sub_type[e412] * sub_type[e125])
                - 2.0 * (sub_type_2[e4] * sub_type_2[e5])
                - 2.0 * (sub_type_2[e423] * sub_type_2[e235])
                - 2.0 * (sub_type_2[e431] * sub_type_2[e315])
                - 2.0 * (sub_type_2[e412] * sub_type_2[e125]),
        );
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for VersorEven {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e1, e2, e3, e5
            self.group3().xyz().with_w(0.0),
        );
        let sub_type_2 = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e321], 2) + f32::powi(sub_type[e1], 2) + f32::powi(sub_type[e2], 2) + f32::powi(sub_type[e3], 2)
                - f32::powi(sub_type_2[e415], 2)
                - f32::powi(sub_type_2[e425], 2)
                - f32::powi(sub_type_2[e435], 2)
                - f32::powi(sub_type_2[e12345], 2),
        );
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for VersorOdd {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(0.0),
        );
        let sub_type_2 = Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(self[e45]),
            // e4235, e4315, e4125, e3215
            self.group3().xyz().with_w(0.0),
        );
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2) + f32::powi(sub_type[scalar], 2)
                - f32::powi(sub_type_2[e45], 2)
                - f32::powi(sub_type_2[e4235], 2)
                - f32::powi(sub_type_2[e4315], 2)
                - f32::powi(sub_type_2[e4125], 2),
        );
    }
}
