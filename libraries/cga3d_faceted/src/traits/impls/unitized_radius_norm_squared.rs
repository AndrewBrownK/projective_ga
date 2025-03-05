// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 43
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         6       9       0
//  Average:         7       8       0
//  Maximum:        46      65       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         6       9       0
//  Average:         7       8       0
//  Maximum:        46      73       1
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        3        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2));
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
        return -(f32::powi(wedge[e415], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e425], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e435], 2) * anti_dot_product[e12345]);
    }
}
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
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
        return -(f32::powi(wedge[e415], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e425], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e435], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        9        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            -f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - f32::powi(self[scalar], 2)
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35]),
        );
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
        return -(f32::powi(wedge[e415], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e425], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e435], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        3        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            -f32::powi(self[scalar], 2) - f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2),
        );
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz());
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
    //      add/sub      mul      div
    // f32       13       12        0
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
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0().with_w(self[e4]).wxyz());
        return -(f32::powi(wedge[e45], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4235], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4315], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4125], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        4        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(self[e321], 2) - f32::powi(self[e1], 2) - f32::powi(self[e2], 2) - f32::powi(self[e3], 2));
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]));
        return -(f32::powi(wedge[e45], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4235], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4315], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4125], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       12        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + 2.0 * (self[e5] * self[e4])
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2),
        );
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]));
        return -(f32::powi(wedge[e45], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4235], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4315], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4125], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        4        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e321], 2) * -1.0);
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz());
        return -(f32::powi(wedge[e4235], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4315], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4125], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiDualNum {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiDualNum {
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[scalar], 2) * f32::powi(self[e1234], -2);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type = Origin::from_groups(/* e4 */ self[e4]);
        let other = Infinity::from_groups(/* e5 */ 1.0);
        return (f32::powi(self[e1], 2) * f32::powi(other[e5], -2) * f32::powi(sub_type[e4], -2))
            + (f32::powi(self[e2], 2) * f32::powi(other[e5], -2) * f32::powi(sub_type[e4], -2))
            + (f32::powi(self[e3], 2) * f32::powi(other[e5], -2) * f32::powi(sub_type[e4], -2));
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        4        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            -f32::powi(self[scalar], 2) - f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2),
        );
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
        return -(f32::powi(wedge[e415], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e425], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e435], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e12345], 2) * anti_dot_product[e12345]);
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
    //      add/sub      mul      div
    // f32        8        9        0
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
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
        return -(f32::powi(wedge[e4235], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4315], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4125], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for CircleAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        9        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2),
        );
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
        return -(f32::powi(wedge[e4235], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4315], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4125], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for CircleAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        9        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ 2.0 * (self[e423] * self[e235]) + 2.0 * (self[e431] * self[e315]) + 2.0 * (self[e412] * self[e125]));
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
        return -(f32::powi(wedge[e4235], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4315], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4125], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for CircleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        3        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2));
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
        return -(f32::powi(wedge[e4235], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4315], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4125], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e423] * self[e235]) + 2.0 * (self[e431] * self[e315]) + 2.0 * (self[e412] * self[e125]) - f32::powi(self[e321], 2),
        );
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz());
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
    //      add/sub      mul      div
    // f32        9        9        0
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
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
        return -(f32::powi(wedge[e4235], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4315], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4125], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        9        0
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
                + f32::powi(self[e12345], 2),
        );
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
        return -(f32::powi(wedge[e4235], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4315], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4125], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        3        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e12345], 2) + f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2),
        );
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz());
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
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
        return -(f32::powi(wedge[e415], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e425], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e435], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e45], 2) - 2.0 * (self[e41] * self[e15]) - 2.0 * (self[e42] * self[e25]) - 2.0 * (self[e43] * self[e35]),
        );
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz());
        return -(f32::powi(wedge[e415], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e425], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e435], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for DipoleAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        9        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -2.0 * (self[e41] * self[e15]) - 2.0 * (self[e42] * self[e25]) - 2.0 * (self[e43] * self[e35]));
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
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
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0().with_w(self[e1234]));
        return -(f32::powi(wedge[e415], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e425], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e435], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e12345], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       12        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e45], 2) + f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2)
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35])
                - 2.0 * (self[e1234] * self[e3215]),
        );
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
        return -(f32::powi(wedge[e415], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e425], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e435], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e12345], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6       12        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            -2.0 * (self[e41] * self[e15]) - 2.0 * (self[e42] * self[e25]) - 2.0 * (self[e43] * self[e35]) - 2.0 * (self[e3215] * self[e1234]),
        );
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
        return -(f32::powi(wedge[e415], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e425], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e435], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e12345], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        4        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e45], 2) + f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2),
        );
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
        return -(f32::powi(wedge[e415], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e425], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e435], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e12345], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       12        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            -f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35])
                - 2.0 * (self[e3215] * self[e1234]),
        );
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
        return -(f32::powi(wedge[e415], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e425], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e435], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e12345], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for DipoleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e45], 2));
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz());
        return -(f32::powi(wedge[e415], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e425], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e435], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        9        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            -f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35]),
        );
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
        return -(f32::powi(wedge[e415], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e425], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e435], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for DualNum {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e12345], 2) * f32::powi(self[e4], -2) * -1.0;
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
    //      f32       46       61        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       46       65        0
    //  no simd       46       73        0
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
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35])
                - 2.0 * (self[e1234] * self[e3215]),
        );
        let sub_type = MultiVector::from_groups(
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
            Simd32x2::from([1.0, other[e5] * sub_type[e1234]]) * Simd32x2::from([0.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e5] * sub_type[e4]),
            // e15, e25, e35
            Simd32x3::from(other[e5]) * sub_type.group1().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (Simd32x3::from(other[e5]) * sub_type.group3().xyz()).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e5] * sub_type[e423], other[e5] * sub_type[e431], other[e5] * sub_type[e412]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]),
            // e3215
            0.0,
        );
        return 2.0 * (anti_dot_product[e12345] * wedge[e41] * wedge[e15])
            + 2.0 * (anti_dot_product[e12345] * wedge[e42] * wedge[e25])
            + 2.0 * (anti_dot_product[e12345] * wedge[e43] * wedge[e35])
            + 2.0 * (anti_dot_product[e12345] * wedge[e1234] * wedge[e3215])
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
        let sub_type = Origin::from_groups(/* e4 */ self[e4]);
        let other = Infinity::from_groups(/* e5 */ 1.0);
        return (f32::powi(self[e1], 2) * f32::powi(other[e5], -2) * f32::powi(sub_type[e4], -2))
            + (f32::powi(self[e2], 2) * f32::powi(other[e5], -2) * f32::powi(sub_type[e4], -2))
            + (f32::powi(self[e3], 2) * f32::powi(other[e5], -2) * f32::powi(sub_type[e4], -2))
            - 2.0 * (self[e4] * self[e5] * f32::powi(other[e5], -2) * f32::powi(sub_type[e4], -2));
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for RoundPointAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self[e5] / (self[e4]) * -2.0;
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
        let sub_type = NullSphereAtOrigin::from_groups(/* e1234 */ self[e1234]);
        let other = Infinity::from_groups(/* e5 */ 1.0);
        return 2.0 * (self[e3215] * self[e1234] * f32::powi(other[e5], -2) * f32::powi(sub_type[e1234], -2))
            - (f32::powi(self[e4235], 2) * f32::powi(other[e5], -2) * f32::powi(sub_type[e1234], -2))
            - (f32::powi(self[e4315], 2) * f32::powi(other[e5], -2) * f32::powi(sub_type[e1234], -2))
            - (f32::powi(self[e4125], 2) * f32::powi(other[e5], -2) * f32::powi(sub_type[e1234], -2));
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for SphereAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self[e3215] / (self[e1234]) * 2.0;
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for SphereOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type = NullSphereAtOrigin::from_groups(/* e1234 */ self[e1234]);
        let other = Infinity::from_groups(/* e5 */ 1.0);
        return -(f32::powi(self[e4235], 2) * f32::powi(other[e5], -2) * f32::powi(sub_type[e1234], -2))
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
    //      add/sub      mul      div
    // f32       14       12        0
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
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]));
        return -(f32::powi(wedge[e45], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4235], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4315], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4125], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       12        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + 2.0 * (self[e4] * self[e5])
                + f32::powi(self[e12345], 2)
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2),
        );
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]));
        return -(f32::powi(wedge[e45], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4235], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4315], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4125], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6       12        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e423] * self[e235]) + 2.0 * (self[e431] * self[e315]) + 2.0 * (self[e412] * self[e125]) + 2.0 * (self[e4] * self[e5]),
        );
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0().wxyz());
        return -(f32::powi(wedge[e45], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4235], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4315], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4125], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        4        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e12345], 2) + f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2),
        );
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]));
        return -(f32::powi(wedge[e45], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4235], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4315], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e4125], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       12        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e423] * self[e235]) + 2.0 * (self[e431] * self[e315]) + 2.0 * (self[e412] * self[e125]) + 2.0 * (self[e5] * self[e4])
                - f32::powi(self[e321], 2)
                - f32::powi(self[e1], 2)
                - f32::powi(self[e2], 2)
                - f32::powi(self[e3], 2),
        );
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]));
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
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
        return -(f32::powi(wedge[e415], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e425], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e435], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e12345], 2) * anti_dot_product[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       12        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            -f32::powi(self[scalar], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35])
                - 2.0 * (self[e3215] * self[e1234]),
        );
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
        return -(f32::powi(wedge[e415], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e425], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e435], 2) * anti_dot_product[e12345])
            - (f32::powi(wedge[e12345], 2) * anti_dot_product[e12345]);
    }
}
