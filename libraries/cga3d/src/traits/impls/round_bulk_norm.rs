// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 17
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       0       0
//  Average:         3       0       0
//  Maximum:        23      16       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       0       0
//  Average:         3       0       0
//  Maximum:        23      16       0
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(0.0),
        );
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2) + f32::powi(sub_type[scalar], 2),
        );
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e1, e2, e3, e5
            self.group3().xyz().with_w(0.0),
        );
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e321], 2) + f32::powi(sub_type[e1], 2) + f32::powi(sub_type[e2], 2) + f32::powi(sub_type[e3], 2),
        );
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiDualNum {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiDualNum {
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[scalar]);
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiFlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiFlatPoint {
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ Simd32x3::from(0.0).with_w(self[e321])[3]);
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiFlector {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e1, e2, e3, e5
            self.group1().xyz().with_w(0.0),
        );
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e321], 2) + f32::powi(sub_type[e1], 2) + f32::powi(sub_type[e2], 2) + f32::powi(sub_type[e3], 2),
        );
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiLine {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiLine::from_groups(/* e23, e31, e12 */ self.group0(), /* e15, e25, e35 */ Simd32x3::from(0.0));
        return Scalar::from_groups(/* scalar */ f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2));
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiMotor {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiMotor::from_groups(/* e23, e31, e12, scalar */ self.group0(), /* e15, e25, e35, e3215 */ Simd32x4::from(0.0));
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2) + f32::powi(sub_type[scalar], 2),
        );
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiPlane {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiPlane::from_groups(/* e1, e2, e3, e5 */ self.group0().xyz().with_w(0.0));
        return Scalar::from_groups(/* scalar */ f32::powi(sub_type[e1], 2) + f32::powi(sub_type[e2], 2) + f32::powi(sub_type[e3], 2));
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for Circle {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for Circle {
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ Simd32x3::from(0.0).with_w(self[e321])[3]);
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for CircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for CircleRotor {
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ Simd32x3::from(0.0).with_w(self[e321])[3]);
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for Dipole {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiLine::from_groups(/* e23, e31, e12 */ self.group1().xyz(), /* e15, e25, e35 */ Simd32x3::from(0.0));
        return Scalar::from_groups(/* scalar */ f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2));
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for DipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiLine::from_groups(/* e23, e31, e12 */ self.group1().xyz(), /* e15, e25, e35 */ Simd32x3::from(0.0));
        return Scalar::from_groups(/* scalar */ f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2));
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for MultiVector {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       23       16        0
    fn round_bulk_norm(self) -> Scalar {
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
        return Scalar::from_groups(
            // scalar
            2.0 * (sub_type[e15] * sub_type[e41])
                + 2.0 * (sub_type[e25] * sub_type[e42])
                + 2.0 * (sub_type[e35] * sub_type[e43])
                + 2.0 * (sub_type[e3215] * sub_type[e1234])
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
        );
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for RoundPoint {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiPlane::from_groups(/* e1, e2, e3, e5 */ self.group0().xyz().with_w(0.0));
        return Scalar::from_groups(/* scalar */ f32::powi(sub_type[e1], 2) + f32::powi(sub_type[e2], 2) + f32::powi(sub_type[e3], 2));
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl std::ops::DivAssign<RoundBulkNormPrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: RoundBulkNormPrefixOrPostfix) {
        *self = self.round_bulk_norm()
    }
}
impl RoundBulkNorm for Scalar {
    fn round_bulk_norm(self) -> Scalar {
        return self;
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for VersorEven {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e1, e2, e3, e5
            self.group3().xyz().with_w(0.0),
        );
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e321], 2) + f32::powi(sub_type[e1], 2) + f32::powi(sub_type[e2], 2) + f32::powi(sub_type[e3], 2),
        );
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for VersorOdd {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(0.0),
        );
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2) + f32::powi(sub_type[scalar], 2),
        );
    }
}
