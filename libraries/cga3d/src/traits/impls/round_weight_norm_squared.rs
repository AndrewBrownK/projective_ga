// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 11
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       0       0
//  Average:         4       3       0
//  Maximum:        24      25       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       0       0
//  Average:         4       8       0
//  Maximum:        24      46       0
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for AntiCircleRotor {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        let wedge = Line::from_groups(/* e415, e425, e435 */ self.group0(), /* e235, e315, e125 */ Simd32x3::from(0.0));
        return AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2));
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for AntiDipoleInversion {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       13        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = AntiDipoleInversion::from_groups(
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
            Simd32x3::from(0.0).with_w(sub_type[e4] * other[e5]),
            // e4235, e4315, e4125, e3215
            other.group0().xx().with_zw(other[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * sub_type.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e45], 2) + f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2),
        );
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for Circle {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        2       12        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        let other = DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([1.0, 0.0]));
        let wedge = Plane::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xx().with_zw(other[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2));
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for CircleRotor {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        2       12        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        let other = DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([1.0, 0.0]));
        let wedge = Plane::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xx().with_zw(other[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2));
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for Dipole {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        let wedge = Line::from_groups(/* e415, e425, e435 */ self.group0(), /* e235, e315, e125 */ Simd32x3::from(0.0));
        return AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2));
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for DipoleInversion {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = DipoleInversion::from_groups(
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
            sub_type.group0().with_w(sub_type[e1234]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
        return AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2) + f32::powi(wedge[e12345], 2),
        );
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for MultiVector {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       18        0
    //    simd4        0        7        0
    // Totals...
    // yes simd       24       25        0
    //  no simd       24       46        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = MultiVector::from_groups(
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
            Simd32x2::from([0.0, (other[e5] * sub_type[e1234]) + (other[e12345] * sub_type[scalar])]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(other[e5]) * sub_type.group1(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0().xx().with_zw(other[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * sub_type.group4().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            other.group0().xx().with_zw(other[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * sub_type.group7().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e1234
            0.0,
        );
        return AntiScalar::from_groups(
            // e12345
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
        );
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for RoundPoint {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for RoundPoint {
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ f32::powi(self[e4], 2));
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for Sphere {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for Sphere {
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ f32::powi(self[e1234], 2));
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for VersorEven {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       13        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = AntiDipoleInversion::from_groups(
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
            Simd32x3::from(0.0).with_w(sub_type[e4] * other[e5]),
            // e4235, e4315, e4125, e3215
            other.group0().xx().with_zw(other[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * sub_type.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e45], 2) + f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2),
        );
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for VersorOdd {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = DipoleInversion::from_groups(
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
            sub_type.group0().with_w(sub_type[e1234]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
        return AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2) + f32::powi(wedge[e12345], 2),
        );
    }
}
