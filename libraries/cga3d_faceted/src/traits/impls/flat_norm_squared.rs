// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 30
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         2       0       0
//   Median:         5       1       0
//  Average:         5       1       0
//  Maximum:        46      41       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         2       0       0
//   Median:         5       3       0
//  Average:         5       3       0
//  Maximum:        46      47       0
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for AntiCircleRotor {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group2().xyz());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([-f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2), f32::powi(self[e45], 2)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1().xyz());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([-f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2), f32::powi(self[e45], 2)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for AntiDipoleInversion {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        1        0
    //  no simd        5        4        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge = FlectorOnOrigin::from_groups(
            // e45, e4235, e4315, e4125
            Simd32x4::from([self[e5], self[e235], self[e315], self[e125]]) * Simd32x4::from(-1.0),
        );
        let sub_type_2 = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1().xyz());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -f32::powi(wedge[e45], 2) - f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2),
                f32::powi(sub_type_2[e415], 2) + f32::powi(sub_type_2[e425], 2) + f32::powi(sub_type_2[e435], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        1        0
    //  no simd        5        4        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group1().with_w(self[e5]).wxyz() * Simd32x4::from(-1.0));
        let sub_type_2 = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -f32::powi(wedge[e45], 2) - f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2),
                f32::powi(sub_type_2[e415], 2) + f32::powi(sub_type_2[e425], 2) + f32::powi(sub_type_2[e435], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        1        0
    //  no simd        5        4        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge = FlectorOnOrigin::from_groups(
            // e45, e4235, e4315, e4125
            Simd32x4::from([self[e5], self[e235], self[e315], self[e125]]) * Simd32x4::from(-1.0),
        );
        let sub_type_2 = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -f32::powi(wedge[e45], 2) - f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2),
                f32::powi(sub_type_2[e415], 2) + f32::powi(sub_type_2[e425], 2) + f32::powi(sub_type_2[e435], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for Circle {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        4        3        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group2() * Simd32x3::from(-1.0));
        let sub_type_2 = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1().xyz());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2),
                f32::powi(sub_type_2[e415], 2) + f32::powi(sub_type_2[e425], 2) + f32::powi(sub_type_2[e435], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for CircleAligningOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        4        3        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group2() * Simd32x3::from(-1.0));
        let sub_type_2 = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2),
                f32::powi(sub_type_2[e415], 2) + f32::powi(sub_type_2[e425], 2) + f32::powi(sub_type_2[e435], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for CircleAtInfinity {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        4        3        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group1() * Simd32x3::from(-1.0));
        let sub_type_2 = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2),
                f32::powi(sub_type_2[e415], 2) + f32::powi(sub_type_2[e425], 2) + f32::powi(sub_type_2[e435], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for CircleRotor {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        5        1        0
    //  no simd        5        3        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group2().xyz() * Simd32x3::from(-1.0));
        let sub_type_2 = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2),
                f32::powi(sub_type_2[e415], 2) + f32::powi(sub_type_2[e425], 2) + f32::powi(sub_type_2[e435], 2) + f32::powi(sub_type_2[e12345], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        5        1        0
    //  no simd        5        3        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group2().xyz() * Simd32x3::from(-1.0));
        let sub_type_2 = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group1().with_w(self[e12345]));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2),
                f32::powi(sub_type_2[e415], 2) + f32::powi(sub_type_2[e425], 2) + f32::powi(sub_type_2[e435], 2) + f32::powi(sub_type_2[e12345], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        5        1        0
    //  no simd        5        3        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group1().xyz() * Simd32x3::from(-1.0));
        let sub_type_2 = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0().with_w(self[e12345]));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2),
                f32::powi(sub_type_2[e415], 2) + f32::powi(sub_type_2[e425], 2) + f32::powi(sub_type_2[e435], 2) + f32::powi(sub_type_2[e12345], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        5        1        0
    //  no simd        5        3        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group1().xyz() * Simd32x3::from(-1.0));
        let sub_type_2 = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2),
                f32::powi(sub_type_2[e415], 2) + f32::powi(sub_type_2[e425], 2) + f32::powi(sub_type_2[e435], 2) + f32::powi(sub_type_2[e12345], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for Dipole {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group2());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([-f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2), f32::powi(self[e45], 2)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([-f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2), f32::powi(self[e45], 2)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for DipoleAtInfinity {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([-f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2), f32::powi(self[e45], 2)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for DipoleInversion {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]));
        let sub_type_2 = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e45], self[e4235], self[e4315], self[e4125]]));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2) - f32::powi(wedge[e12345], 2),
                f32::powi(sub_type_2[e45], 2) + f32::powi(sub_type_2[e4235], 2) + f32::powi(sub_type_2[e4315], 2) + f32::powi(sub_type_2[e4125], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]));
        let sub_type_2 = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e45], self[e4235], self[e4315], self[e4125]]));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2) - f32::powi(wedge[e12345], 2),
                f32::powi(sub_type_2[e45], 2) + f32::powi(sub_type_2[e4235], 2) + f32::powi(sub_type_2[e4315], 2) + f32::powi(sub_type_2[e4125], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group1().with_w(self[e3215]));
        let sub_type_2 = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e45], self[e4235], self[e4315], self[e4125]]));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2) - f32::powi(wedge[e12345], 2),
                f32::powi(sub_type_2[e45], 2) + f32::powi(sub_type_2[e4235], 2) + f32::powi(sub_type_2[e4315], 2) + f32::powi(sub_type_2[e4125], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for FlatPoint {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([-f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2), f32::powi(self[e45], 2)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for Flector {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]));
        let sub_type_2 = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e45], self[e4235], self[e4315], self[e4125]]));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2) - f32::powi(wedge[e12345], 2),
                f32::powi(sub_type_2[e45], 2) + f32::powi(sub_type_2[e4235], 2) + f32::powi(sub_type_2[e4315], 2) + f32::powi(sub_type_2[e4125], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for Line {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        4        3        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group1() * Simd32x3::from(-1.0));
        let sub_type_2 = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2),
                f32::powi(sub_type_2[e415], 2) + f32::powi(sub_type_2[e425], 2) + f32::powi(sub_type_2[e435], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for Motor {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        1        0
    //  no simd        6        4        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group1().wxyz() * Simd32x4::from(-1.0));
        let sub_type_2 = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -f32::powi(wedge[e45], 2) - f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2),
                f32::powi(sub_type_2[e415], 2) + f32::powi(sub_type_2[e425], 2) + f32::powi(sub_type_2[e435], 2) + f32::powi(sub_type_2[e12345], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl std::ops::DivAssign<FlatNormSquaredPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: FlatNormSquaredPrefixOrPostfix) {
        *self = self.flat_norm_squared()
    }
}
impl FlatNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       46       38        0
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       46       41        0
    //  no simd       46       47        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let sub_type = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            self[e3215],
        );
        let other = Origin::from_groups(/* e4 */ 1.0);
        let wedge = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, sub_type[e3215] * other[e4]]) * Simd32x2::from([0.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(sub_type[e5] * other[e4] * -1.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (Simd32x3::from(other[e4]) * sub_type.group4()).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, sub_type[e235] * other[e4], sub_type[e315] * other[e4], sub_type[e125] * other[e4]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            0.0,
        );
        let sub_type_2 = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(self[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group6().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            0.0,
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                2.0 * (wedge[e41] * wedge[e15])
                    + 2.0 * (wedge[e42] * wedge[e25])
                    + 2.0 * (wedge[e43] * wedge[e35])
                    + 2.0 * (wedge[e1234] * wedge[e3215])
                    + f32::powi(wedge[scalar], 2)
                    + f32::powi(wedge[e1], 2)
                    + f32::powi(wedge[e2], 2)
                    + f32::powi(wedge[e3], 2)
                    + f32::powi(wedge[e23], 2)
                    + f32::powi(wedge[e31], 2)
                    + f32::powi(wedge[e12], 2)
                    + f32::powi(wedge[e321], 2)
                    - f32::powi(wedge[e12345], 2)
                    - f32::powi(wedge[e45], 2)
                    - f32::powi(wedge[e415], 2)
                    - f32::powi(wedge[e425], 2)
                    - f32::powi(wedge[e435], 2)
                    - f32::powi(wedge[e4235], 2)
                    - f32::powi(wedge[e4315], 2)
                    - f32::powi(wedge[e4125], 2)
                    - 2.0 * (wedge[e4] * wedge[e5])
                    - 2.0 * (wedge[e423] * wedge[e235])
                    - 2.0 * (wedge[e431] * wedge[e315])
                    - 2.0 * (wedge[e412] * wedge[e125]),
                2.0 * (sub_type_2[e4] * sub_type_2[e5])
                    + 2.0 * (sub_type_2[e423] * sub_type_2[e235])
                    + 2.0 * (sub_type_2[e431] * sub_type_2[e315])
                    + 2.0 * (sub_type_2[e412] * sub_type_2[e125])
                    + f32::powi(sub_type_2[e12345], 2)
                    + f32::powi(sub_type_2[e45], 2)
                    + f32::powi(sub_type_2[e415], 2)
                    + f32::powi(sub_type_2[e425], 2)
                    + f32::powi(sub_type_2[e435], 2)
                    + f32::powi(sub_type_2[e4235], 2)
                    + f32::powi(sub_type_2[e4315], 2)
                    + f32::powi(sub_type_2[e4125], 2)
                    - f32::powi(sub_type_2[scalar], 2)
                    - f32::powi(sub_type_2[e1], 2)
                    - f32::powi(sub_type_2[e2], 2)
                    - f32::powi(sub_type_2[e3], 2)
                    - f32::powi(sub_type_2[e23], 2)
                    - f32::powi(sub_type_2[e31], 2)
                    - f32::powi(sub_type_2[e12], 2)
                    - f32::powi(sub_type_2[e321], 2)
                    - 2.0 * (sub_type_2[e41] * sub_type_2[e15])
                    - 2.0 * (sub_type_2[e42] * sub_type_2[e25])
                    - 2.0 * (sub_type_2[e43] * sub_type_2[e35])
                    - 2.0 * (sub_type_2[e1234] * sub_type_2[e3215]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for Plane {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        2        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let sub_type_2 = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                f32::powi(self[e3215], 2),
                f32::powi(sub_type_2[e4235], 2) + f32::powi(sub_type_2[e4315], 2) + f32::powi(sub_type_2[e4125], 2),
            ]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for Sphere {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        2        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let sub_type_2 = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                f32::powi(self[e3215], 2),
                f32::powi(sub_type_2[e4235], 2) + f32::powi(sub_type_2[e4315], 2) + f32::powi(sub_type_2[e4125], 2),
            ]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for VersorEven {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        1        0
    //  no simd        6        4        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group2().wxyz() * Simd32x4::from(-1.0));
        let sub_type_2 = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -f32::powi(wedge[e45], 2) - f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2),
                f32::powi(sub_type_2[e415], 2) + f32::powi(sub_type_2[e425], 2) + f32::powi(sub_type_2[e435], 2) + f32::powi(sub_type_2[e12345], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        1        0
    //  no simd        6        4        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group2().wxyz() * Simd32x4::from(-1.0));
        let sub_type_2 = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -f32::powi(wedge[e45], 2) - f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2),
                f32::powi(sub_type_2[e415], 2) + f32::powi(sub_type_2[e425], 2) + f32::powi(sub_type_2[e435], 2) + f32::powi(sub_type_2[e12345], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        1        0
    //  no simd        6        4        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group2().wxyz() * Simd32x4::from(-1.0));
        let sub_type_2 = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -f32::powi(wedge[e45], 2) - f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2),
                f32::powi(sub_type_2[e415], 2) + f32::powi(sub_type_2[e425], 2) + f32::powi(sub_type_2[e435], 2) + f32::powi(sub_type_2[e12345], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for VersorOdd {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]));
        let sub_type_2 = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e45], self[e4235], self[e4315], self[e4125]]));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2) - f32::powi(wedge[e12345], 2),
                f32::powi(sub_type_2[e45], 2) + f32::powi(sub_type_2[e4235], 2) + f32::powi(sub_type_2[e4315], 2) + f32::powi(sub_type_2[e4125], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]));
        let sub_type_2 = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e45], self[e4235], self[e4315], self[e4125]]));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2) - f32::powi(wedge[e12345], 2),
                f32::powi(sub_type_2[e45], 2) + f32::powi(sub_type_2[e4235], 2) + f32::powi(sub_type_2[e4315], 2) + f32::powi(sub_type_2[e4125], 2),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
