// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 10
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         2       0       0
//   Median:         5       3       0
//  Average:         8       5       0
//  Maximum:        47      41       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         2       0       0
//   Median:         5      12       0
//  Average:         8      11       0
//  Maximum:        47      62       0
impl std::ops::Div<RoundNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        let sub_type = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(0.0),
        );
        let wedge = Line::from_groups(/* e415, e425, e435 */ self.group0(), /* e235, e315, e125 */ Simd32x3::from(0.0));
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
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Div<RoundNormPrefixOrPostfix> for AntiDipoleInversion {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        6        4        0
    //  no simd        6       13        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        let sub_type = AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e1, e2, e3, e5
            self.group3().xyz().with_w(0.0),
        );
        let sub_type_2 = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
        let other = DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([1.0, 0.0]));
        let wedge = Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(sub_type_2[e4] * other[e5]),
            // e4235, e4315, e4125, e3215
            other.group0().xx().with_zw(other[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * sub_type_2.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
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
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Div<RoundNormPrefixOrPostfix> for Circle {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        2       12        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        let other = DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([1.0, 0.0]));
        let wedge = Plane::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xx().with_zw(other[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[e321], f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Div<RoundNormPrefixOrPostfix> for CircleRotor {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        2       12        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        let other = DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([1.0, 0.0]));
        let wedge = Plane::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xx().with_zw(other[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[e321], f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Div<RoundNormPrefixOrPostfix> for Dipole {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        let sub_type = AntiLine::from_groups(/* e23, e31, e12 */ self.group1().xyz(), /* e15, e25, e35 */ Simd32x3::from(0.0));
        let wedge = Line::from_groups(/* e415, e425, e435 */ self.group0(), /* e235, e315, e125 */ Simd32x3::from(0.0));
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
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Div<RoundNormPrefixOrPostfix> for DipoleInversion {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        let sub_type = AntiLine::from_groups(/* e23, e31, e12 */ self.group1().xyz(), /* e15, e25, e35 */ Simd32x3::from(0.0));
        let sub_type_2 = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
        let wedge = Motor::from_groups(
            // e415, e425, e435, e12345
            sub_type_2.group0().with_w(sub_type_2[e1234]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
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
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Div<RoundNormPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl std::ops::DivAssign<RoundNormPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: RoundNormPrefixOrPostfix) {
        *self = self.round_norm()
    }
}
impl RoundNorm for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       47       34        0
    //    simd4        0        7        0
    // Totals...
    // yes simd       47       41        0
    //  no simd       47       62        0
    fn round_norm(self) -> MultiVector {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            self[e1234],
        );
        let other = DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([1.0, 0.0]));
        let wedge = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other[e5] * sub_type_2[e1234]) + (other[e12345] * sub_type_2[scalar])]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(other[e5]) * sub_type_2.group1(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0().xx().with_zw(other[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * sub_type_2.group4().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            other.group0().xx().with_zw(other[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * sub_type_2.group7().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e1234
            0.0,
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
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
                    - 2.0 * (wedge[e15] * wedge[e41])
                    - 2.0 * (wedge[e25] * wedge[e42])
                    - 2.0 * (wedge[e35] * wedge[e43])
                    - 2.0 * (wedge[e3215] * wedge[e1234]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Div<RoundNormPrefixOrPostfix> for RoundPoint {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        let sub_type = AntiPlane::from_groups(/* e1, e2, e3, e5 */ self.group0().xyz().with_w(0.0));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([f32::powi(sub_type[e1], 2) + f32::powi(sub_type[e2], 2) + f32::powi(sub_type[e3], 2), self[e4]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Div<RoundNormPrefixOrPostfix> for VersorEven {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        6        4        0
    //  no simd        6       13        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        let sub_type = AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e1, e2, e3, e5
            self.group3().xyz().with_w(0.0),
        );
        let sub_type_2 = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
        let other = DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([1.0, 0.0]));
        let wedge = Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(sub_type_2[e4] * other[e5]),
            // e4235, e4315, e4125, e3215
            other.group0().xx().with_zw(other[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * sub_type_2.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
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
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Div<RoundNormPrefixOrPostfix> for VersorOdd {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        let sub_type = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(0.0),
        );
        let sub_type_2 = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
        let wedge = Motor::from_groups(
            // e415, e425, e435, e12345
            sub_type_2.group0().with_w(sub_type_2[e1234]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
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
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
