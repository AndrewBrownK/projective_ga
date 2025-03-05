// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 16
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3      11       0
//  Average:         4       9       0
//  Maximum:        15      28       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         6      14       0
//  Average:         7      16       0
//  Maximum:        34      51       0
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        2        4        0
    // Totals...
    // yes simd        4        4        0
    //  no simd        8       12        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type = FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group2().xyz().with_w(0.0));
        let other = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        let wedge = Line::from_groups(
            // e415, e425, e435
            (Simd32x3::from(other[e4]) * sub_type.group0().xyz()) - (Simd32x3::from(sub_type[e45]) * other.group0().xyz()),
            // e235, e315, e125
            (sub_type.group0().zxy() * other.group0().yzx()) - (sub_type.group0().yzx() * other.group0().zxy()),
        );
        let sub_type_2 = FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x3::from(0.0).with_w(self[e45]));
        return -(f32::powi(wedge[e415], 2) * f32::powi(sub_type_2[e45], -2))
            - (f32::powi(wedge[e425], 2) * f32::powi(sub_type_2[e45], -2))
            - (f32::powi(wedge[e435], 2) * f32::powi(sub_type_2[e45], -2));
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        9        0
    //    simd3        2        8        0
    // Totals...
    // yes simd        5       17        0
    //  no simd        9       33        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group2().xyz().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(self[e5]),
        );
        let other = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        let wedge = DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e4]) * sub_type.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            ((sub_type.group1().yzx() * other.group0().zxy()) - (sub_type.group1().zxy() * other.group0().yzx())).with_w(sub_type[e5] * other[e4] * -1.0),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e5]) * sub_type.group1().xyz()) - (Simd32x3::from(sub_type[e5]) * other.group0().xyz())).with_w(0.0),
            // e4235, e4315, e4125, e3215
            (Simd32x3::from(other[e4]) * sub_type.group0().xyz() * Simd32x3::from(-1.0))
                .with_w((sub_type[e235] * other[e1]) + (sub_type[e315] * other[e2]) + (sub_type[e125] * other[e3]) + (sub_type[e321] * other[e5])),
        );
        return f32::powi(self.group1().xyz()[0], 2) * wedge[e41] * wedge[e15] * 2.0;
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for Circle {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        9        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        3       11        0
    //  no simd        3       15        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group2().with_w(0.0));
        let other = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        let wedge = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x3::from(other[e4]) * sub_type.group0().xyz() * Simd32x3::from(-1.0))
                .with_w((sub_type[e235] * other[e1]) + (sub_type[e315] * other[e2]) + (sub_type[e125] * other[e3]) + (sub_type[e321] * other[e5])),
            // e1234
            sub_type[e321] * other[e4] * -1.0,
        );
        return f32::powi(self.group1().xyz()[0], 2) * wedge[e3215] * wedge[e1234] * 2.0;
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3       11        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group2().xyz().with_w(0.0));
        let other = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        return f32::powi(
            (Simd32x3::from(other[e4]) * sub_type.group0().xyz() * Simd32x3::from(-1.0))
                .with_w((sub_type[e235] * other[e1]) + (sub_type[e315] * other[e2]) + (sub_type[e125] * other[e3]) + (sub_type[e321] * other[e5]))[0],
            2,
        ) * f32::powi(self[e415], 2)
            * -1.0;
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for Dipole {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        2        4        0
    // Totals...
    // yes simd        4        4        0
    //  no simd        8       12        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type = FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group2().with_w(0.0));
        let other = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        let wedge = Line::from_groups(
            // e415, e425, e435
            (Simd32x3::from(other[e4]) * sub_type.group0().xyz()) - (Simd32x3::from(sub_type[e45]) * other.group0().xyz()),
            // e235, e315, e125
            (sub_type.group0().zxy() * other.group0().yzx()) - (sub_type.group0().yzx() * other.group0().zxy()),
        );
        let sub_type_2 = FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x3::from(0.0).with_w(self[e45]));
        return -(f32::powi(wedge[e415], 2) * f32::powi(sub_type_2[e45], -2))
            - (f32::powi(wedge[e425], 2) * f32::powi(sub_type_2[e45], -2))
            - (f32::powi(wedge[e435], 2) * f32::powi(sub_type_2[e45], -2));
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       10        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        3       11        0
    //  no simd        6       14        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type = Flector::from_groups(
            // e15, e25, e35, e45
            self.group2().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
        );
        let other = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        return f32::powi(
            (Simd32x4::from([
                sub_type[e45] * other[e1] * -1.0,
                sub_type[e45] * other[e2] * -1.0,
                sub_type[e45] * other[e3] * -1.0,
                (sub_type[e4315] * other[e2]) + (sub_type[e4125] * other[e3]) + (sub_type[e3215] * other[e4]),
            ]) + (other.group0().wwwx() * sub_type.group0().xyz().with_w(sub_type[e4235])))[0],
            2,
        ) * f32::powi(Simd32x3::from(0.0).with_w(self[e45])[3], 2)
            * -1.0;
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for DualNum {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for DualNum {
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self[e5], 2) * f32::powi(self[e12345], -2);
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for FlatPoint {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        2        4        0
    // Totals...
    // yes simd        4        4        0
    //  no simd        8       12        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type = FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0().xyz().with_w(0.0));
        let other = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        let wedge = Line::from_groups(
            // e415, e425, e435
            (Simd32x3::from(other[e4]) * sub_type.group0().xyz()) - (Simd32x3::from(sub_type[e45]) * other.group0().xyz()),
            // e235, e315, e125
            (sub_type.group0().zxy() * other.group0().yzx()) - (sub_type.group0().yzx() * other.group0().zxy()),
        );
        let sub_type_2 = FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x3::from(0.0).with_w(self[e45]));
        return -(f32::powi(wedge[e415], 2) * f32::powi(sub_type_2[e45], -2))
            - (f32::powi(wedge[e425], 2) * f32::powi(sub_type_2[e45], -2))
            - (f32::powi(wedge[e435], 2) * f32::powi(sub_type_2[e45], -2));
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for Flector {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       10        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        3       11        0
    //  no simd        6       14        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type = Flector::from_groups(
            // e15, e25, e35, e45
            self.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
        );
        let other = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        return f32::powi(
            (Simd32x4::from([
                sub_type[e45] * other[e1] * -1.0,
                sub_type[e45] * other[e2] * -1.0,
                sub_type[e45] * other[e3] * -1.0,
                (sub_type[e4315] * other[e2]) + (sub_type[e4125] * other[e3]) + (sub_type[e3215] * other[e4]),
            ]) + (other.group0().wwwx() * sub_type.group0().xyz().with_w(sub_type[e4235])))[0],
            2,
        ) * f32::powi(Simd32x3::from(0.0).with_w(self[e45])[3], 2)
            * -1.0;
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for Line {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        9        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        3       11        0
    //  no simd        3       15        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group1().with_w(0.0));
        let other = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        let wedge = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x3::from(other[e4]) * sub_type.group0().xyz() * Simd32x3::from(-1.0))
                .with_w((sub_type[e235] * other[e1]) + (sub_type[e315] * other[e2]) + (sub_type[e125] * other[e3]) + (sub_type[e321] * other[e5])),
            // e1234
            sub_type[e321] * other[e4] * -1.0,
        );
        return f32::powi(self[e415], 2) * wedge[e3215] * wedge[e1234] * 2.0;
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for Motor {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        9        0
    //    simd3        2        8        0
    // Totals...
    // yes simd        5       17        0
    //  no simd        9       33        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group1().xyz().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(self[e5]),
        );
        let other = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        let wedge = DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e4]) * sub_type.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            ((sub_type.group1().yzx() * other.group0().zxy()) - (sub_type.group1().zxy() * other.group0().yzx())).with_w(sub_type[e5] * other[e4] * -1.0),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e5]) * sub_type.group1().xyz()) - (Simd32x3::from(sub_type[e5]) * other.group0().xyz())).with_w(0.0),
            // e4235, e4315, e4125, e3215
            (Simd32x3::from(other[e4]) * sub_type.group0().xyz() * Simd32x3::from(-1.0))
                .with_w((sub_type[e235] * other[e1]) + (sub_type[e315] * other[e2]) + (sub_type[e125] * other[e3]) + (sub_type[e321] * other[e5])),
        );
        return f32::powi(self[e415], 2) * wedge[e41] * wedge[e15] * 2.0;
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       18        0
    //    simd3        2        7        0
    //    simd4        5        3        0
    // Totals...
    // yes simd       15       28        0
    //  no simd       34       51        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3().xyz().with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
            // e1234
            0.0,
        );
        let other = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        let wedge = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (sub_type[e4235] * other[e1]) + (sub_type[e4315] * other[e2]) + (sub_type[e4125] * other[e3]) + (sub_type[e3215] * other[e4]) + (sub_type[e1234] * other[e5]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            (Simd32x4::from(other[e5]) * sub_type.group1()) - (Simd32x4::from(sub_type[e5]) * other.group0()),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (Simd32x3::from(other[e4]) * sub_type.group3().xyz()).with_w(0.0) + (Simd32x3::from(other[e5]) * sub_type.group4()).with_w(0.0)
                - (Simd32x3::from(sub_type[e45]) * other.group0().xyz()).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from(other[e5]) * sub_type.group5()) + (sub_type.group3().zxy() * other.group0().yzx()) - (sub_type.group3().yzx() * other.group0().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(sub_type[e425] * other[e3]) - (sub_type[e235] * other[e4]),
                -(sub_type[e435] * other[e1]) - (sub_type[e315] * other[e4]),
                -(sub_type[e415] * other[e2]) - (sub_type[e125] * other[e4]),
                (sub_type[e321] * other[e5]) + (sub_type[e125] * other[e3]),
            ]) + (other.group0().yzxy() * sub_type.group6().zxy().with_w(sub_type[e315]))
                + (Simd32x3::from(other[e5]) * sub_type.group7()).with_w(sub_type[e235] * other[e1]),
            // e1234
            0.0,
        );
        let sub_type_2 = MultiVector::from_groups(
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
        return sub_type_2[e423] * sub_type_2[e235] * wedge[e15] * wedge[e41] * 4.0;
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for Plane {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        let wedge = AntiScalar::from_groups(/* e12345 */ Simd32x3::from(0.0).with_w(1.0)[3] * self[e3215]);
        let sub_type_2 = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0().xyz().with_w(0.0));
        return -(f32::powi(wedge[e12345], 2) * f32::powi(sub_type_2[e4235], 2))
            - (f32::powi(wedge[e12345], 2) * f32::powi(sub_type_2[e4315], 2))
            - (f32::powi(wedge[e12345], 2) * f32::powi(sub_type_2[e4125], 2));
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for Sphere {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        let wedge = AntiScalar::from_groups(/* e12345 */ Simd32x3::from(0.0).with_w(1.0)[3] * self[e3215]);
        let sub_type_2 = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0().xyz().with_w(0.0));
        return -(f32::powi(wedge[e12345], 2) * f32::powi(sub_type_2[e4235], 2))
            - (f32::powi(wedge[e12345], 2) * f32::powi(sub_type_2[e4315], 2))
            - (f32::powi(wedge[e12345], 2) * f32::powi(sub_type_2[e4125], 2));
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        9        0
    //    simd3        2        8        0
    // Totals...
    // yes simd        5       17        0
    //  no simd        9       33        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type = AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group2().xyz().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(self[e5]),
        );
        let other = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        let wedge = DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e4]) * sub_type.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            ((sub_type.group1().yzx() * other.group0().zxy()) - (sub_type.group1().zxy() * other.group0().yzx())).with_w(sub_type[e5] * other[e4] * -1.0),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e5]) * sub_type.group1().xyz()) - (Simd32x3::from(sub_type[e5]) * other.group0().xyz())).with_w(0.0),
            // e4235, e4315, e4125, e3215
            (Simd32x3::from(other[e4]) * sub_type.group0().xyz() * Simd32x3::from(-1.0))
                .with_w((sub_type[e235] * other[e1]) + (sub_type[e315] * other[e2]) + (sub_type[e125] * other[e3]) + (sub_type[e321] * other[e5])),
        );
        return f32::powi(self[e415], 2) * wedge[e41] * wedge[e15] * 2.0;
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       10        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        3       11        0
    //  no simd        6       14        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type = Flector::from_groups(
            // e15, e25, e35, e45
            self.group2().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
        );
        let other = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        return f32::powi(
            (Simd32x4::from([
                sub_type[e45] * other[e1] * -1.0,
                sub_type[e45] * other[e2] * -1.0,
                sub_type[e45] * other[e3] * -1.0,
                (sub_type[e4315] * other[e2]) + (sub_type[e4125] * other[e3]) + (sub_type[e3215] * other[e4]),
            ]) + (other.group0().wwwx() * sub_type.group0().xyz().with_w(sub_type[e4235])))[0],
            2,
        ) * f32::powi(Simd32x3::from(0.0).with_w(self[e45])[3], 2)
            * -1.0;
    }
}
