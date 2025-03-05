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
//   Median:         3       0       0
//  Average:         4       9       0
//  Maximum:        23      89       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       0       0
//  Average:         4      10       0
//  Maximum:        23      97       0
impl std::ops::Div<UnitizedCenterNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn unitized_center_norm(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]));
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
        return (f32::powi(sub_type[e23], 2) * f32::powi(wedge[e415], 2))
            + (f32::powi(sub_type[e31], 2) * f32::powi(wedge[e415], 2))
            + (f32::powi(sub_type[e12], 2) * f32::powi(wedge[e415], 2))
            + (f32::powi(sub_type[scalar], 2) * f32::powi(wedge[e415], 2));
    }
}
impl std::ops::Div<UnitizedCenterNormPrefixOrPostfix> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn unitized_center_norm(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]));
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0().with_w(self[e4]).wxyz());
        return (f32::powi(sub_type[e321], 2) * f32::powi(wedge[e45], 2))
            + (f32::powi(sub_type[e1], 2) * f32::powi(wedge[e45], 2))
            + (f32::powi(sub_type[e2], 2) * f32::powi(wedge[e45], 2))
            + (f32::powi(sub_type[e3], 2) * f32::powi(wedge[e45], 2));
    }
}
impl std::ops::Div<UnitizedCenterNormPrefixOrPostfix> for Circle {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for Circle {
    fn unitized_center_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e423], 2) * f32::powi(self[e321], 2);
    }
}
impl std::ops::Div<UnitizedCenterNormPrefixOrPostfix> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for CircleRotor {
    fn unitized_center_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e423], 2) * f32::powi(self[e321], 2);
    }
}
impl std::ops::Div<UnitizedCenterNormPrefixOrPostfix> for Dipole {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_center_norm(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group1().xyz());
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
        return (f32::powi(sub_type[e23], 2) * f32::powi(wedge[e415], 2))
            + (f32::powi(sub_type[e31], 2) * f32::powi(wedge[e415], 2))
            + (f32::powi(sub_type[e12], 2) * f32::powi(wedge[e415], 2));
    }
}
impl std::ops::Div<UnitizedCenterNormPrefixOrPostfix> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_center_norm(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group1().xyz());
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0().with_w(self[e1234]));
        return (f32::powi(sub_type[e23], 2) * f32::powi(wedge[e415], 2))
            + (f32::powi(sub_type[e31], 2) * f32::powi(wedge[e415], 2))
            + (f32::powi(sub_type[e12], 2) * f32::powi(wedge[e415], 2));
    }
}
impl std::ops::Div<UnitizedCenterNormPrefixOrPostfix> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       85        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       23       89        0
    //  no simd       23       97        0
    fn unitized_center_norm(self) -> f32 {
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
        let sub_type_3 = MultiVector::from_groups(
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
            Simd32x2::from([1.0, other[e5] * sub_type_3[e1234]]) * Simd32x2::from([0.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e5] * sub_type_3[e4]),
            // e15, e25, e35
            Simd32x3::from(other[e5]) * sub_type_3.group1().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (Simd32x3::from(other[e5]) * sub_type_3.group3().xyz()).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e5] * sub_type_3[e423], other[e5] * sub_type_3[e431], other[e5] * sub_type_3[e412]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]),
            // e3215
            0.0,
        );
        return 4.0 * (sub_type[e41] * sub_type[e15] * wedge[e4] * wedge[e5])
            + 4.0 * (sub_type[e42] * sub_type[e25] * wedge[e4] * wedge[e5])
            + 4.0 * (sub_type[e43] * sub_type[e35] * wedge[e4] * wedge[e5])
            + 4.0 * (sub_type[e1234] * sub_type[e3215] * wedge[e4] * wedge[e5])
            + 2.0 * (f32::powi(sub_type[scalar], 2) * wedge[e4] * wedge[e5])
            + 2.0 * (f32::powi(sub_type[e1], 2) * wedge[e4] * wedge[e5])
            + 2.0 * (f32::powi(sub_type[e2], 2) * wedge[e4] * wedge[e5])
            + 2.0 * (f32::powi(sub_type[e3], 2) * wedge[e4] * wedge[e5])
            + 2.0 * (f32::powi(sub_type[e23], 2) * wedge[e4] * wedge[e5])
            + 2.0 * (f32::powi(sub_type[e31], 2) * wedge[e4] * wedge[e5])
            + 2.0 * (f32::powi(sub_type[e12], 2) * wedge[e4] * wedge[e5])
            + 2.0 * (f32::powi(sub_type[e321], 2) * wedge[e4] * wedge[e5])
            - 2.0 * (f32::powi(sub_type[e12345], 2) * wedge[e4] * wedge[e5])
            - 2.0 * (f32::powi(sub_type[e45], 2) * wedge[e4] * wedge[e5])
            - 2.0 * (f32::powi(sub_type[e415], 2) * wedge[e4] * wedge[e5])
            - 2.0 * (f32::powi(sub_type[e425], 2) * wedge[e4] * wedge[e5])
            - 2.0 * (f32::powi(sub_type[e435], 2) * wedge[e4] * wedge[e5])
            - 2.0 * (f32::powi(sub_type[e4235], 2) * wedge[e4] * wedge[e5])
            - 2.0 * (f32::powi(sub_type[e4315], 2) * wedge[e4] * wedge[e5])
            - 2.0 * (f32::powi(sub_type[e4125], 2) * wedge[e4] * wedge[e5])
            - 4.0 * (sub_type[e4] * sub_type[e5] * wedge[e4] * wedge[e5])
            - 4.0 * (sub_type[e423] * sub_type[e235] * wedge[e4] * wedge[e5])
            - 4.0 * (sub_type[e431] * sub_type[e315] * wedge[e4] * wedge[e5])
            - 4.0 * (sub_type[e412] * sub_type[e125] * wedge[e4] * wedge[e5]);
    }
}
impl std::ops::Div<UnitizedCenterNormPrefixOrPostfix> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn unitized_center_norm(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]));
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]));
        return (f32::powi(sub_type[e321], 2) * f32::powi(wedge[e45], 2))
            + (f32::powi(sub_type[e1], 2) * f32::powi(wedge[e45], 2))
            + (f32::powi(sub_type[e2], 2) * f32::powi(wedge[e45], 2))
            + (f32::powi(sub_type[e3], 2) * f32::powi(wedge[e45], 2));
    }
}
impl std::ops::Div<UnitizedCenterNormPrefixOrPostfix> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn unitized_center_norm(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]));
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
        return (f32::powi(sub_type[e23], 2) * f32::powi(wedge[e415], 2))
            + (f32::powi(sub_type[e31], 2) * f32::powi(wedge[e415], 2))
            + (f32::powi(sub_type[e12], 2) * f32::powi(wedge[e415], 2))
            + (f32::powi(sub_type[scalar], 2) * f32::powi(wedge[e415], 2));
    }
}
