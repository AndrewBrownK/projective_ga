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
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for AntiCircleRotor {
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ Simd32x3::from(0.0).with_w(self[e45])[3]);
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for AntiDipoleInversion {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = Line::from_groups(/* e415, e425, e435 */ self.group1().xyz(), /* e235, e315, e125 */ Simd32x3::from(0.0));
        return AntiScalar::from_groups(/* e12345 */ f32::powi(sub_type[e415], 2) + f32::powi(sub_type[e425], 2) + f32::powi(sub_type[e435], 2));
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl std::ops::DivAssign<FlatWeightNormPrefixOrPostfix> for AntiScalar {
    fn div_assign(&mut self, _rhs: FlatWeightNormPrefixOrPostfix) {
        *self = self.flat_weight_norm()
    }
}
impl FlatWeightNorm for AntiScalar {
    fn flat_weight_norm(self) -> AntiScalar {
        return self;
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for Circle {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = Line::from_groups(/* e415, e425, e435 */ self.group1().xyz(), /* e235, e315, e125 */ Simd32x3::from(0.0));
        return AntiScalar::from_groups(/* e12345 */ f32::powi(sub_type[e415], 2) + f32::powi(sub_type[e425], 2) + f32::powi(sub_type[e435], 2));
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for CircleRotor {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
        return AntiScalar::from_groups(
            // e12345
            f32::powi(sub_type[e415], 2) + f32::powi(sub_type[e425], 2) + f32::powi(sub_type[e435], 2) + f32::powi(sub_type[e12345], 2),
        );
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for Dipole {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for Dipole {
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ Simd32x3::from(0.0).with_w(self[e45])[3]);
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for DipoleInversion {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(self[e45]),
            // e4235, e4315, e4125, e3215
            self.group3().xyz().with_w(0.0),
        );
        return AntiScalar::from_groups(
            // e12345
            f32::powi(sub_type[e45], 2) + f32::powi(sub_type[e4235], 2) + f32::powi(sub_type[e4315], 2) + f32::powi(sub_type[e4125], 2),
        );
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for DualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for DualNum {
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e12345]);
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for FlatPoint {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for FlatPoint {
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ Simd32x3::from(0.0).with_w(self[e45])[3]);
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for Flector {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(self[e45]),
            // e4235, e4315, e4125, e3215
            self.group1().xyz().with_w(0.0),
        );
        return AntiScalar::from_groups(
            // e12345
            f32::powi(sub_type[e45], 2) + f32::powi(sub_type[e4235], 2) + f32::powi(sub_type[e4315], 2) + f32::powi(sub_type[e4125], 2),
        );
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for Line {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = Line::from_groups(/* e415, e425, e435 */ self.group0(), /* e235, e315, e125 */ Simd32x3::from(0.0));
        return AntiScalar::from_groups(/* e12345 */ f32::powi(sub_type[e415], 2) + f32::powi(sub_type[e425], 2) + f32::powi(sub_type[e435], 2));
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for Motor {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = Motor::from_groups(/* e415, e425, e435, e12345 */ self.group0(), /* e235, e315, e125, e5 */ Simd32x4::from(0.0));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(sub_type[e415], 2) + f32::powi(sub_type[e425], 2) + f32::powi(sub_type[e435], 2) + f32::powi(sub_type[e12345], 2),
        );
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for MultiVector {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       23       16        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = MultiVector::from_groups(
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
        return AntiScalar::from_groups(
            // e12345
            2.0 * (sub_type[e4] * sub_type[e5])
                + 2.0 * (sub_type[e423] * sub_type[e235])
                + 2.0 * (sub_type[e431] * sub_type[e315])
                + 2.0 * (sub_type[e412] * sub_type[e125])
                + f32::powi(sub_type[e12345], 2)
                + f32::powi(sub_type[e45], 2)
                + f32::powi(sub_type[e415], 2)
                + f32::powi(sub_type[e425], 2)
                + f32::powi(sub_type[e435], 2)
                + f32::powi(sub_type[e4235], 2)
                + f32::powi(sub_type[e4315], 2)
                + f32::powi(sub_type[e4125], 2)
                - f32::powi(sub_type[scalar], 2)
                - f32::powi(sub_type[e1], 2)
                - f32::powi(sub_type[e2], 2)
                - f32::powi(sub_type[e3], 2)
                - f32::powi(sub_type[e23], 2)
                - f32::powi(sub_type[e31], 2)
                - f32::powi(sub_type[e12], 2)
                - f32::powi(sub_type[e321], 2)
                - 2.0 * (sub_type[e15] * sub_type[e41])
                - 2.0 * (sub_type[e25] * sub_type[e42])
                - 2.0 * (sub_type[e35] * sub_type[e43])
                - 2.0 * (sub_type[e3215] * sub_type[e1234]),
        );
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for Plane {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0().xyz().with_w(0.0));
        return AntiScalar::from_groups(/* e12345 */ f32::powi(sub_type[e4235], 2) + f32::powi(sub_type[e4315], 2) + f32::powi(sub_type[e4125], 2));
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for Sphere {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0().xyz().with_w(0.0));
        return AntiScalar::from_groups(/* e12345 */ f32::powi(sub_type[e4235], 2) + f32::powi(sub_type[e4315], 2) + f32::powi(sub_type[e4125], 2));
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for VersorEven {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
        return AntiScalar::from_groups(
            // e12345
            f32::powi(sub_type[e415], 2) + f32::powi(sub_type[e425], 2) + f32::powi(sub_type[e435], 2) + f32::powi(sub_type[e12345], 2),
        );
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for VersorOdd {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(self[e45]),
            // e4235, e4315, e4125, e3215
            self.group3().xyz().with_w(0.0),
        );
        return AntiScalar::from_groups(
            // e12345
            f32::powi(sub_type[e45], 2) + f32::powi(sub_type[e4235], 2) + f32::powi(sub_type[e4315], 2) + f32::powi(sub_type[e4125], 2),
        );
    }
}
