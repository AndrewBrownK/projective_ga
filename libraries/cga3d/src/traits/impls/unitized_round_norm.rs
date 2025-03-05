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
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       2       0
//  Maximum:         2      13       3
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       6       0
//  Maximum:         2      34       3
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiCircleRotor {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e41], 2) * f32::powi(self[e23], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiDipoleInversion {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e321], 2) * f32::powi(self[e4], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for Circle {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2       15        0
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x3::from(0.0).with_w(self[e321]));
        let other = DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([1.0, 0.0]));
        let wedge = Plane::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xx().with_zw(other[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return (f32::powi(wedge[e4235], 2) * sub_type[e321]) + (f32::powi(wedge[e4315], 2) * sub_type[e321]) + (f32::powi(wedge[e4125], 2) * sub_type[e321]);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2       15        0
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x3::from(0.0).with_w(self[e321]));
        let other = DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([1.0, 0.0]));
        let wedge = Plane::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xx().with_zw(other[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return (f32::powi(wedge[e4235], 2) * sub_type[e321]) + (f32::powi(wedge[e4315], 2) * sub_type[e321]) + (f32::powi(wedge[e4125], 2) * sub_type[e321]);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for Dipole {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for Dipole {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e41], 2) * f32::powi(self[e23], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for DipoleInversion {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e41], 2) * f32::powi(self[e23], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        6        0
    //    simd4        0        7        0
    // Totals...
    // yes simd        1       13        0
    //  no simd        1       34        0
    fn unitized_round_norm(self) -> f32 {
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
        return (sub_type[e15] * sub_type[e41] * wedge[e4] * wedge[e5]) * 4.0;
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for RoundPoint {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        3
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiPlane::from_groups(/* e1, e2, e3, e5 */ self.group0().xyz().with_w(0.0));
        let wedge = FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x3::from(0.0).with_w(self[e4]));
        return (f32::powi(sub_type[e1], 2) / (wedge[e45])) + (f32::powi(sub_type[e2], 2) / (wedge[e45])) + (f32::powi(sub_type[e3], 2) / (wedge[e45]));
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for VersorEven {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e321], 2) * f32::powi(self[e4], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for VersorOdd {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e41], 2) * f32::powi(self[e23], 2);
    }
}
