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
//  Minimum:         3       2       0
//   Median:         9      12       0
//  Average:        12      15       0
//  Maximum:        47      65       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         3       2       0
//   Median:         9      12       0
//  Average:        12      20       0
//  Maximum:        47      86       0
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9        9        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e45], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - f32::powi(self[scalar], 2)
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35]),
        );
        let wedge = Line::from_groups(/* e415, e425, e435 */ self.group0(), /* e235, e315, e125 */ Simd32x3::from(0.0));
        return -(f32::powi(wedge[e415], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e425], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e435], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       13        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       13       16        0
    //  no simd       13       25        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + 2.0 * (self[e4] * self[e5])
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2)
                - f32::powi(self[e321], 2)
                - f32::powi(self[e1], 2)
                - f32::powi(self[e2], 2)
                - f32::powi(self[e3], 2),
        );
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
        return -(f32::powi(wedge[e45], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4235], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4315], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4125], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for Circle {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8        9        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        8       12        0
    //  no simd        8       21        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2)
                - f32::powi(self[e321], 2),
        );
        let other = DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([1.0, 0.0]));
        let wedge = Plane::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xx().with_zw(other[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return -(f32::powi(wedge[e4235], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4315], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4125], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9        9        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        9       12        0
    //  no simd        9       21        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2)
                + f32::powi(self[e12345], 2)
                - f32::powi(self[e321], 2),
        );
        let other = DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([1.0, 0.0]));
        let wedge = Plane::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xx().with_zw(other[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return -(f32::powi(wedge[e4235], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4315], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4125], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for Dipole {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        9        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e45], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35]),
        );
        let wedge = Line::from_groups(/* e415, e425, e435 */ self.group0(), /* e235, e315, e125 */ Simd32x3::from(0.0));
        return -(f32::powi(wedge[e415], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e425], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e435], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       12        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e45], 2) + f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35])
                - 2.0 * (self[e1234] * self[e3215]),
        );
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
        return -(f32::powi(wedge[e415], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e425], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e435], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e12345], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       47       58        0
    //    simd4        0        7        0
    // Totals...
    // yes simd       47       65        0
    //  no simd       47       86        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e4] * self[e5])
                + 2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + f32::powi(self[e12345], 2)
                + f32::powi(self[e45], 2)
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2)
                + f32::powi(self[e4235], 2)
                + f32::powi(self[e4315], 2)
                + f32::powi(self[e4125], 2)
                - f32::powi(self[scalar], 2)
                - f32::powi(self[e1], 2)
                - f32::powi(self[e2], 2)
                - f32::powi(self[e3], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - f32::powi(self[e321], 2)
                - 2.0 * (self[e15] * self[e41])
                - 2.0 * (self[e25] * self[e42])
                - 2.0 * (self[e35] * self[e43])
                - 2.0 * (self[e3215] * self[e1234]),
        );
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
        return 2.0 * (anti_dot_product[e12345] * wedge[e15] * wedge[e41])
            + 2.0 * (anti_dot_product[e12345] * wedge[e25] * wedge[e42])
            + 2.0 * (anti_dot_product[e12345] * wedge[e35] * wedge[e43])
            + 2.0 * (anti_dot_product[e12345] * wedge[e3215] * wedge[e1234])
            + (f32::powi(wedge[scalar], 2) * anti_dot_product[e12345])
            + (f32::powi(wedge[e1], 2) * anti_dot_product[e12345])
            + (f32::powi(wedge[e2], 2) * anti_dot_product[e12345])
            + (f32::powi(wedge[e3], 2) * anti_dot_product[e12345])
            + (f32::powi(wedge[e23], 2) * anti_dot_product[e12345])
            + (f32::powi(wedge[e31], 2) * anti_dot_product[e12345])
            + (f32::powi(wedge[e12], 2) * anti_dot_product[e12345])
            + (f32::powi(wedge[e321], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e12345], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e45], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e415], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e425], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e435], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4235], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4315], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4125], 2) * anti_dot_product[e12345])
            - 2.0 * (anti_dot_product[e12345] * wedge[e4] * wedge[e5])
            - 2.0 * (anti_dot_product[e12345] * wedge[e423] * wedge[e235])
            - 2.0 * (anti_dot_product[e12345] * wedge[e431] * wedge[e315])
            - 2.0 * (anti_dot_product[e12345] * wedge[e412] * wedge[e125]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for RoundPoint {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        2        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(self[e4]), /* e5 */ 0.0);
        let other = DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([1.0, 0.0]));
        return (f32::powi(self[e1], 2) * f32::powi(other[e5], -2) * f32::powi(sub_type[e4], -2))
            + (f32::powi(self[e2], 2) * f32::powi(other[e5], -2) * f32::powi(sub_type[e4], -2))
            + (f32::powi(self[e3], 2) * f32::powi(other[e5], -2) * f32::powi(sub_type[e4], -2))
            - 2.0 * (self[e4] * self[e5] * f32::powi(other[e5], -2) * f32::powi(sub_type[e4], -2));
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for Sphere {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        2        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type = Sphere::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(0.0), /* e1234 */ self[e1234]);
        let other = DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([1.0, 0.0]));
        return 2.0 * (self[e3215] * self[e1234] * f32::powi(other[e5], -2) * f32::powi(sub_type[e1234], -2))
            - (f32::powi(self[e4235], 2) * f32::powi(other[e5], -2) * f32::powi(sub_type[e1234], -2))
            - (f32::powi(self[e4315], 2) * f32::powi(other[e5], -2) * f32::powi(sub_type[e1234], -2))
            - (f32::powi(self[e4125], 2) * f32::powi(other[e5], -2) * f32::powi(sub_type[e1234], -2));
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       13        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       14       16        0
    //  no simd       14       25        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + 2.0 * (self[e5] * self[e4])
                + f32::powi(self[e12345], 2)
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2)
                - f32::powi(self[e321], 2)
                - f32::powi(self[e1], 2)
                - f32::powi(self[e2], 2)
                - f32::powi(self[e3], 2),
        );
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
        return -(f32::powi(wedge[e45], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4235], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4315], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4125], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       12        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e45], 2) + f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2)
                - f32::powi(self[scalar], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35])
                - 2.0 * (self[e1234] * self[e3215]),
        );
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
        return -(f32::powi(wedge[e415], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e425], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e435], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e12345], 2) * anti_dot_product[e12345]);
    }
}
