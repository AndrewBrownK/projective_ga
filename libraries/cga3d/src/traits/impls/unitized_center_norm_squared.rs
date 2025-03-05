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
//  Minimum:         0       0       0
//   Median:         3       3       0
//  Average:         4      11       0
//  Maximum:        24      89       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3      12       0
//  Average:         4      17       0
//  Maximum:        24     110       0
impl std::ops::Div<UnitizedCenterNormSquaredPrefixOrPostfix> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm_squared()
    }
}
impl UnitizedCenterNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(0.0),
        );
        let wedge = Line::from_groups(/* e415, e425, e435 */ self.group0(), /* e235, e315, e125 */ Simd32x3::from(0.0));
        return (f32::powi(sub_type[e23], 2) * f32::powi(wedge[e415], 2))
            + (f32::powi(sub_type[e31], 2) * f32::powi(wedge[e415], 2))
            + (f32::powi(sub_type[e12], 2) * f32::powi(wedge[e415], 2))
            + (f32::powi(sub_type[scalar], 2) * f32::powi(wedge[e415], 2));
    }
}
impl std::ops::Div<UnitizedCenterNormSquaredPrefixOrPostfix> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm_squared()
    }
}
impl UnitizedCenterNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       13        0
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e1, e2, e3, e5
            self.group3().xyz().with_w(0.0),
        );
        let sub_type_3 = AntiDipoleInversion::from_groups(
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
            Simd32x3::from(0.0).with_w(sub_type_3[e4] * other[e5]),
            // e4235, e4315, e4125, e3215
            other.group0().xx().with_zw(other[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * sub_type_3.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return (f32::powi(sub_type[e321], 2) * f32::powi(wedge[e45], 2))
            + (f32::powi(sub_type[e1], 2) * f32::powi(wedge[e45], 2))
            + (f32::powi(sub_type[e2], 2) * f32::powi(wedge[e45], 2))
            + (f32::powi(sub_type[e3], 2) * f32::powi(wedge[e45], 2));
    }
}
impl std::ops::Div<UnitizedCenterNormSquaredPrefixOrPostfix> for Circle {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm_squared()
    }
}
impl UnitizedCenterNormSquared for Circle {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        let other = DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([1.0, 0.0]));
        return f32::powi(
            (other.group0().xx().with_zw(other[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]))[0],
            2,
        ) * f32::powi(Simd32x3::from(0.0).with_w(self[e321])[3], 2);
    }
}
impl std::ops::Div<UnitizedCenterNormSquaredPrefixOrPostfix> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm_squared()
    }
}
impl UnitizedCenterNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        let other = DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([1.0, 0.0]));
        return f32::powi(
            (other.group0().xx().with_zw(other[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]))[0],
            2,
        ) * f32::powi(Simd32x3::from(0.0).with_w(self[e321])[3], 2);
    }
}
impl std::ops::Div<UnitizedCenterNormSquaredPrefixOrPostfix> for Dipole {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm_squared()
    }
}
impl UnitizedCenterNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiLine::from_groups(/* e23, e31, e12 */ self.group1().xyz(), /* e15, e25, e35 */ Simd32x3::from(0.0));
        let wedge = Line::from_groups(/* e415, e425, e435 */ self.group0(), /* e235, e315, e125 */ Simd32x3::from(0.0));
        return (f32::powi(sub_type[e23], 2) * f32::powi(wedge[e415], 2))
            + (f32::powi(sub_type[e31], 2) * f32::powi(wedge[e415], 2))
            + (f32::powi(sub_type[e12], 2) * f32::powi(wedge[e415], 2));
    }
}
impl std::ops::Div<UnitizedCenterNormSquaredPrefixOrPostfix> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm_squared()
    }
}
impl UnitizedCenterNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiLine::from_groups(/* e23, e31, e12 */ self.group1().xyz(), /* e15, e25, e35 */ Simd32x3::from(0.0));
        let sub_type_3 = DipoleInversion::from_groups(
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
            sub_type_3.group0().with_w(sub_type_3[e1234]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
        return (f32::powi(sub_type[e23], 2) * f32::powi(wedge[e415], 2))
            + (f32::powi(sub_type[e31], 2) * f32::powi(wedge[e415], 2))
            + (f32::powi(sub_type[e12], 2) * f32::powi(wedge[e415], 2));
    }
}
impl std::ops::Div<UnitizedCenterNormSquaredPrefixOrPostfix> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm_squared()
    }
}
impl UnitizedCenterNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       82        0
    //    simd4        0        7        0
    // Totals...
    // yes simd       24       89        0
    //  no simd       24      110        0
    fn unitized_center_norm_squared(self) -> f32 {
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
        let sub_type_3 = MultiVector::from_groups(
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
            Simd32x2::from([0.0, (other[e5] * sub_type_3[e1234]) + (other[e12345] * sub_type_3[scalar])]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(other[e5]) * sub_type_3.group1(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0().xx().with_zw(other[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * sub_type_3.group4().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            other.group0().xx().with_zw(other[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * sub_type_3.group7().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e1234
            0.0,
        );
        return 4.0 * (sub_type[e15] * sub_type[e41] * wedge[e423] * wedge[e235])
            + 4.0 * (sub_type[e25] * sub_type[e42] * wedge[e423] * wedge[e235])
            + 4.0 * (sub_type[e35] * sub_type[e43] * wedge[e423] * wedge[e235])
            + 4.0 * (sub_type[e3215] * sub_type[e1234] * wedge[e423] * wedge[e235])
            + 2.0 * (f32::powi(sub_type[scalar], 2) * wedge[e423] * wedge[e235])
            + 2.0 * (f32::powi(sub_type[e1], 2) * wedge[e423] * wedge[e235])
            + 2.0 * (f32::powi(sub_type[e2], 2) * wedge[e423] * wedge[e235])
            + 2.0 * (f32::powi(sub_type[e3], 2) * wedge[e423] * wedge[e235])
            + 2.0 * (f32::powi(sub_type[e23], 2) * wedge[e423] * wedge[e235])
            + 2.0 * (f32::powi(sub_type[e31], 2) * wedge[e423] * wedge[e235])
            + 2.0 * (f32::powi(sub_type[e12], 2) * wedge[e423] * wedge[e235])
            + 2.0 * (f32::powi(sub_type[e321], 2) * wedge[e423] * wedge[e235])
            - 2.0 * (f32::powi(sub_type[e12345], 2) * wedge[e423] * wedge[e235])
            - 2.0 * (f32::powi(sub_type[e45], 2) * wedge[e423] * wedge[e235])
            - 2.0 * (f32::powi(sub_type[e415], 2) * wedge[e423] * wedge[e235])
            - 2.0 * (f32::powi(sub_type[e425], 2) * wedge[e423] * wedge[e235])
            - 2.0 * (f32::powi(sub_type[e435], 2) * wedge[e423] * wedge[e235])
            - 2.0 * (f32::powi(sub_type[e4235], 2) * wedge[e423] * wedge[e235])
            - 2.0 * (f32::powi(sub_type[e4315], 2) * wedge[e423] * wedge[e235])
            - 2.0 * (f32::powi(sub_type[e4125], 2) * wedge[e423] * wedge[e235])
            - 4.0 * (sub_type[e4] * sub_type[e5] * wedge[e423] * wedge[e235])
            - 4.0 * (sub_type[e423] * sub_type[e235] * wedge[e423] * wedge[e235])
            - 4.0 * (sub_type[e431] * sub_type[e315] * wedge[e423] * wedge[e235])
            - 4.0 * (sub_type[e412] * sub_type[e125] * wedge[e423] * wedge[e235]);
    }
}
impl std::ops::Div<UnitizedCenterNormSquaredPrefixOrPostfix> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm_squared()
    }
}
impl UnitizedCenterNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       13        0
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e1, e2, e3, e5
            self.group3().xyz().with_w(0.0),
        );
        let sub_type_3 = AntiDipoleInversion::from_groups(
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
            Simd32x3::from(0.0).with_w(sub_type_3[e4] * other[e5]),
            // e4235, e4315, e4125, e3215
            other.group0().xx().with_zw(other[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * sub_type_3.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return (f32::powi(sub_type[e321], 2) * f32::powi(wedge[e45], 2))
            + (f32::powi(sub_type[e1], 2) * f32::powi(wedge[e45], 2))
            + (f32::powi(sub_type[e2], 2) * f32::powi(wedge[e45], 2))
            + (f32::powi(sub_type[e3], 2) * f32::powi(wedge[e45], 2));
    }
}
impl std::ops::Div<UnitizedCenterNormSquaredPrefixOrPostfix> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm_squared()
    }
}
impl UnitizedCenterNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(0.0),
        );
        let sub_type_3 = DipoleInversion::from_groups(
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
            sub_type_3.group0().with_w(sub_type_3[e1234]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
        return (f32::powi(sub_type[e23], 2) * f32::powi(wedge[e415], 2))
            + (f32::powi(sub_type[e31], 2) * f32::powi(wedge[e415], 2))
            + (f32::powi(sub_type[e12], 2) * f32::powi(wedge[e415], 2))
            + (f32::powi(sub_type[scalar], 2) * f32::powi(wedge[e415], 2));
    }
}
