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
//   Median:         6       6       0
//  Average:         7       5       0
//  Maximum:        46      41       3
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         6       6       0
//  Average:         7       5       0
//  Maximum:        46      49       3
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2));
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
        return -(f32::powi(wedge[e415], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e425], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e435], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9        6        0
    fn unitized_radius_norm(self) -> f32 {
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
        return -(f32::powi(wedge[e415], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e425], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e435], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        6        0
    fn unitized_radius_norm(self) -> f32 {
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
        return -(f32::powi(wedge[e415], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e425], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e435], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            -f32::powi(self[scalar], 2) - f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2),
        );
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz());
        return -(f32::powi(wedge[e415], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e425], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e435], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13        8        0
    fn unitized_radius_norm(self) -> f32 {
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
        return -(f32::powi(wedge[e45], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4235], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4315], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4125], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(self[e321], 2) - f32::powi(self[e1], 2) - f32::powi(self[e2], 2) - f32::powi(self[e3], 2));
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]));
        return -(f32::powi(wedge[e45], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4235], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4315], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4125], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9        8        0
    fn unitized_radius_norm(self) -> f32 {
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
        return -(f32::powi(wedge[e45], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4235], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4315], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4125], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e321], 2) * -1.0);
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz());
        return -(f32::powi(wedge[e4235], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4315], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4125], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiDualNum {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        return self[scalar] / (self[e1234]);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        3
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        return (f32::powi(self[e1], 2) / (self[e4])) + (f32::powi(self[e2], 2) / (self[e4])) + (f32::powi(self[e3], 2) / (self[e4]));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            -f32::powi(self[scalar], 2) - f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2),
        );
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
        return -(f32::powi(wedge[e415], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e425], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e435], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e12345], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for Circle {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        6        0
    fn unitized_radius_norm(self) -> f32 {
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
        return -(f32::powi(wedge[e4235], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4315], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4125], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for CircleAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        6        0
    fn unitized_radius_norm(self) -> f32 {
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
        return -(f32::powi(wedge[e4235], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4315], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4125], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for CircleAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        6        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ 2.0 * (self[e423] * self[e235]) + 2.0 * (self[e431] * self[e315]) + 2.0 * (self[e412] * self[e125]));
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
        return -(f32::powi(wedge[e4235], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4315], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4125], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for CircleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2));
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
        return -(f32::powi(wedge[e4235], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4315], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4125], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e423] * self[e235]) + 2.0 * (self[e431] * self[e315]) + 2.0 * (self[e412] * self[e125]) - f32::powi(self[e321], 2),
        );
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz());
        return -(f32::powi(wedge[e4235], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4315], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4125], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9        6        0
    fn unitized_radius_norm(self) -> f32 {
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
        return -(f32::powi(wedge[e4235], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4315], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4125], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        6        0
    fn unitized_radius_norm(self) -> f32 {
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
        return -(f32::powi(wedge[e4235], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4315], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4125], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e12345], 2) + f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2),
        );
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz());
        return -(f32::powi(wedge[e4235], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4315], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4125], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for Dipole {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        6        0
    fn unitized_radius_norm(self) -> f32 {
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
        return -(f32::powi(wedge[e415], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e425], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e435], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e45], 2) - 2.0 * (self[e41] * self[e15]) - 2.0 * (self[e42] * self[e25]) - 2.0 * (self[e43] * self[e35]),
        );
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz());
        return -(f32::powi(wedge[e415], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e425], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e435], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for DipoleAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        6        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -2.0 * (self[e41] * self[e15]) - 2.0 * (self[e42] * self[e25]) - 2.0 * (self[e43] * self[e35]));
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
        return -(f32::powi(wedge[e415], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e425], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e435], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13        8        0
    fn unitized_radius_norm(self) -> f32 {
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
        return -(f32::powi(wedge[e415], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e425], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e435], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e12345], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        8        0
    fn unitized_radius_norm(self) -> f32 {
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
        return -(f32::powi(wedge[e415], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e425], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e435], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e12345], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        8        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            -2.0 * (self[e41] * self[e15]) - 2.0 * (self[e42] * self[e25]) - 2.0 * (self[e43] * self[e35]) - 2.0 * (self[e3215] * self[e1234]),
        );
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
        return -(f32::powi(wedge[e415], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e425], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e435], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e12345], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e45], 2) + f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2),
        );
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
        return -(f32::powi(wedge[e415], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e425], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e435], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e12345], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9        8        0
    fn unitized_radius_norm(self) -> f32 {
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
        return -(f32::powi(wedge[e415], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e425], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e435], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e12345], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for DipoleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e45], 2));
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz());
        return -(f32::powi(wedge[e415], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e425], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e435], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        6        0
    fn unitized_radius_norm(self) -> f32 {
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
        return -(f32::powi(wedge[e415], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e425], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e435], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for DualNum {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        return self[e12345] / (self[e4]) * -1.0;
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       46       37        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       46       41        0
    //  no simd       46       49        0
    fn unitized_radius_norm(self) -> f32 {
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
        return 2.0 * (wedge[e41] * wedge[e15] * f32::powf(anti_dot_product[e12345], 0.5))
            + 2.0 * (wedge[e42] * wedge[e25] * f32::powf(anti_dot_product[e12345], 0.5))
            + 2.0 * (wedge[e43] * wedge[e35] * f32::powf(anti_dot_product[e12345], 0.5))
            + 2.0 * (wedge[e1234] * wedge[e3215] * f32::powf(anti_dot_product[e12345], 0.5))
            + (f32::powi(wedge[scalar], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            + (f32::powi(wedge[e1], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            + (f32::powi(wedge[e2], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            + (f32::powi(wedge[e3], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            + (f32::powi(wedge[e23], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            + (f32::powi(wedge[e31], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            + (f32::powi(wedge[e12], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            + (f32::powi(wedge[e321], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e12345], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e45], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e415], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e425], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e435], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4235], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4315], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4125], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - 2.0 * (wedge[e4] * wedge[e5] * f32::powf(anti_dot_product[e12345], 0.5))
            - 2.0 * (wedge[e423] * wedge[e235] * f32::powf(anti_dot_product[e12345], 0.5))
            - 2.0 * (wedge[e431] * wedge[e315] * f32::powf(anti_dot_product[e12345], 0.5))
            - 2.0 * (wedge[e412] * wedge[e125] * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for RoundPoint {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        3
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        return (f32::powi(self[e1], 2) / (self[e4])) + (f32::powi(self[e2], 2) / (self[e4])) + (f32::powi(self[e3], 2) / (self[e4])) - 2.0 * self[e5];
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for RoundPointAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powf(self[e5], 0.5) * f32::powf(self[e4], -0.5) * -2.0;
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for Sphere {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        3
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        return 2.0 * self[e3215] - (f32::powi(self[e4235], 2) / (self[e1234])) - (f32::powi(self[e4315], 2) / (self[e1234])) - (f32::powi(self[e4125], 2) / (self[e1234]));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for SphereAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powf(self[e3215], 0.5) * f32::powf(self[e1234], -0.5) * 2.0;
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for SphereOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        3
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        return -(f32::powi(self[e4235], 2) / (self[e1234])) - (f32::powi(self[e4315], 2) / (self[e1234])) - (f32::powi(self[e4125], 2) / (self[e1234]));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14        8        0
    fn unitized_radius_norm(self) -> f32 {
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
        return -(f32::powi(wedge[e45], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4235], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4315], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4125], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        8        0
    fn unitized_radius_norm(self) -> f32 {
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
        return -(f32::powi(wedge[e45], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4235], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4315], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4125], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        8        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e423] * self[e235]) + 2.0 * (self[e431] * self[e315]) + 2.0 * (self[e412] * self[e125]) + 2.0 * (self[e4] * self[e5]),
        );
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0().wxyz());
        return -(f32::powi(wedge[e45], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4235], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4315], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4125], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e12345], 2) + f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2),
        );
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]));
        return -(f32::powi(wedge[e45], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4235], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4315], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4125], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        8        0
    fn unitized_radius_norm(self) -> f32 {
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
        return -(f32::powi(wedge[e45], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4235], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4315], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e4125], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14        8        0
    fn unitized_radius_norm(self) -> f32 {
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
        return -(f32::powi(wedge[e415], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e425], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e435], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e12345], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        8        0
    fn unitized_radius_norm(self) -> f32 {
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
        return -(f32::powi(wedge[e415], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e425], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e435], 2) * f32::powf(anti_dot_product[e12345], 0.5))
            - (f32::powi(wedge[e12345], 2) * f32::powf(anti_dot_product[e12345], 0.5));
    }
}
