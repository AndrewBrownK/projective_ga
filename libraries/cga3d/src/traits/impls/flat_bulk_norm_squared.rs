// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 23
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         5       6       0
//  Average:         7       9       0
//  Maximum:        38      40       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         8      12       0
//  Average:        10      16       0
//  Maximum:        57      63       0
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for AntiCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        2        4        0
    // Totals...
    // yes simd        4        4        0
    //  no simd        8       12        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type = FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group2().xyz().with_w(0.0));
        let other = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        let wedge = Line::from_groups(
            // e415, e425, e435
            (Simd32x3::from(other[e4]) * sub_type.group0().xyz()) - (Simd32x3::from(sub_type[e45]) * other.group0().xyz()),
            // e235, e315, e125
            (sub_type.group0().zxy() * other.group0().yzx()) - (sub_type.group0().yzx() * other.group0().zxy()),
        );
        return Scalar::from_groups(/* scalar */ -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2));
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for AntiDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       14        0
    //    simd3        2        8        0
    // Totals...
    // yes simd       15       22        0
    //  no simd       19       38        0
    fn flat_bulk_norm_squared(self) -> Scalar {
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
        return Scalar::from_groups(
            // scalar
            2.0 * (wedge[e41] * wedge[e15])
                + 2.0 * (wedge[e42] * wedge[e25])
                + 2.0 * (wedge[e43] * wedge[e35])
                + 2.0 * (wedge[e1234] * wedge[e3215])
                + f32::powi(wedge[e23], 2)
                + f32::powi(wedge[e31], 2)
                + f32::powi(wedge[e12], 2)
                - f32::powi(wedge[e45], 2)
                - f32::powi(wedge[e4235], 2)
                - f32::powi(wedge[e4315], 2)
                - f32::powi(wedge[e4125], 2),
        );
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for AntiDualNum {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powi(self[e3215], 2) * -1.0);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for AntiFlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        4        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        6        0
    //  no simd        5       10        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group0().xyz().with_w(0.0));
        let other = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        let wedge = Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x3::from(other[e4]) * sub_type.group0().xyz() * Simd32x3::from(-1.0))
                .with_w((sub_type[e235] * other[e1]) + (sub_type[e315] * other[e2]) + (sub_type[e125] * other[e3]) + (sub_type[e321] * other[e5])),
        );
        return Scalar::from_groups(/* scalar */ -f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2));
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for AntiFlector {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       14        0
    //    simd3        2        8        0
    // Totals...
    // yes simd       15       22        0
    //  no simd       19       38        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group0().xyz().with_w(0.0),
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
        return Scalar::from_groups(
            // scalar
            2.0 * (wedge[e41] * wedge[e15])
                + 2.0 * (wedge[e42] * wedge[e25])
                + 2.0 * (wedge[e43] * wedge[e35])
                + 2.0 * (wedge[e1234] * wedge[e3215])
                + f32::powi(wedge[e23], 2)
                + f32::powi(wedge[e31], 2)
                + f32::powi(wedge[e12], 2)
                - f32::powi(wedge[e45], 2)
                - f32::powi(wedge[e4235], 2)
                - f32::powi(wedge[e4315], 2)
                - f32::powi(wedge[e4125], 2),
        );
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for AntiLine {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        2        4        0
    // Totals...
    // yes simd        4        4        0
    //  no simd        8       12        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type = FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group1().with_w(0.0));
        let other = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        let wedge = Line::from_groups(
            // e415, e425, e435
            (Simd32x3::from(other[e4]) * sub_type.group0().xyz()) - (Simd32x3::from(sub_type[e45]) * other.group0().xyz()),
            // e235, e315, e125
            (sub_type.group0().zxy() * other.group0().yzx()) - (sub_type.group0().yzx() * other.group0().zxy()),
        );
        return Scalar::from_groups(/* scalar */ -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2));
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for AntiMotor {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        7       12        0
    //  no simd       12       19        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type = Flector::from_groups(
            // e15, e25, e35, e45
            self.group1().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
        );
        let other = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        let wedge = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                sub_type[e45] * other[e1] * -1.0,
                sub_type[e45] * other[e2] * -1.0,
                sub_type[e45] * other[e3] * -1.0,
                (sub_type[e4315] * other[e2]) + (sub_type[e4125] * other[e3]) + (sub_type[e3215] * other[e4]),
            ]) + (other.group0().wwwx() * sub_type.group0().xyz().with_w(sub_type[e4235])),
            // e235, e315, e125, e5
            ((sub_type.group0().zxy() * other.group0().yzx()) - (sub_type.group0().yzx() * other.group0().zxy())).with_w(0.0),
        );
        return Scalar::from_groups(
            // scalar
            -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2) - f32::powi(wedge[e12345], 2),
        );
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for AntiPlane {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for AntiPlane {
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powi(self[e5], 2));
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for Circle {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        6       10        0
    //  no simd        6       14        0
    fn flat_bulk_norm_squared(self) -> Scalar {
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
        return Scalar::from_groups(
            // scalar
            2.0 * (wedge[e3215] * wedge[e1234]) - f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2),
        );
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for CircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        4        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        6        0
    //  no simd        5       10        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group2().xyz().with_w(0.0));
        let other = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        let wedge = Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x3::from(other[e4]) * sub_type.group0().xyz() * Simd32x3::from(-1.0))
                .with_w((sub_type[e235] * other[e1]) + (sub_type[e315] * other[e2]) + (sub_type[e125] * other[e3]) + (sub_type[e321] * other[e5])),
        );
        return Scalar::from_groups(/* scalar */ -f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2));
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for Dipole {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        2        4        0
    // Totals...
    // yes simd        4        4        0
    //  no simd        8       12        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type = FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group2().with_w(0.0));
        let other = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        let wedge = Line::from_groups(
            // e415, e425, e435
            (Simd32x3::from(other[e4]) * sub_type.group0().xyz()) - (Simd32x3::from(sub_type[e45]) * other.group0().xyz()),
            // e235, e315, e125
            (sub_type.group0().zxy() * other.group0().yzx()) - (sub_type.group0().yzx() * other.group0().zxy()),
        );
        return Scalar::from_groups(/* scalar */ -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2));
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for DipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        7       12        0
    //  no simd       12       19        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type = Flector::from_groups(
            // e15, e25, e35, e45
            self.group2().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
        );
        let other = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        let wedge = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                sub_type[e45] * other[e1] * -1.0,
                sub_type[e45] * other[e2] * -1.0,
                sub_type[e45] * other[e3] * -1.0,
                (sub_type[e4315] * other[e2]) + (sub_type[e4125] * other[e3]) + (sub_type[e3215] * other[e4]),
            ]) + (other.group0().wwwx() * sub_type.group0().xyz().with_w(sub_type[e4235])),
            // e235, e315, e125, e5
            ((sub_type.group0().zxy() * other.group0().yzx()) - (sub_type.group0().yzx() * other.group0().zxy())).with_w(0.0),
        );
        return Scalar::from_groups(
            // scalar
            -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2) - f32::powi(wedge[e12345], 2),
        );
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for DualNum {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for DualNum {
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powi(self[e5], 2));
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for FlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        2        4        0
    // Totals...
    // yes simd        4        4        0
    //  no simd        8       12        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type = FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0().xyz().with_w(0.0));
        let other = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        let wedge = Line::from_groups(
            // e415, e425, e435
            (Simd32x3::from(other[e4]) * sub_type.group0().xyz()) - (Simd32x3::from(sub_type[e45]) * other.group0().xyz()),
            // e235, e315, e125
            (sub_type.group0().zxy() * other.group0().yzx()) - (sub_type.group0().yzx() * other.group0().zxy()),
        );
        return Scalar::from_groups(/* scalar */ -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2));
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for Flector {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        7       12        0
    //  no simd       12       19        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type = Flector::from_groups(
            // e15, e25, e35, e45
            self.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
        );
        let other = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        let wedge = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                sub_type[e45] * other[e1] * -1.0,
                sub_type[e45] * other[e2] * -1.0,
                sub_type[e45] * other[e3] * -1.0,
                (sub_type[e4315] * other[e2]) + (sub_type[e4125] * other[e3]) + (sub_type[e3215] * other[e4]),
            ]) + (other.group0().wwwx() * sub_type.group0().xyz().with_w(sub_type[e4235])),
            // e235, e315, e125, e5
            ((sub_type.group0().zxy() * other.group0().yzx()) - (sub_type.group0().yzx() * other.group0().zxy())).with_w(0.0),
        );
        return Scalar::from_groups(
            // scalar
            -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2) - f32::powi(wedge[e12345], 2),
        );
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for Line {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        6       10        0
    //  no simd        6       14        0
    fn flat_bulk_norm_squared(self) -> Scalar {
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
        return Scalar::from_groups(
            // scalar
            2.0 * (wedge[e3215] * wedge[e1234]) - f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2),
        );
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for Motor {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       14        0
    //    simd3        2        8        0
    // Totals...
    // yes simd       15       22        0
    //  no simd       19       38        0
    fn flat_bulk_norm_squared(self) -> Scalar {
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
        return Scalar::from_groups(
            // scalar
            2.0 * (wedge[e41] * wedge[e15])
                + 2.0 * (wedge[e42] * wedge[e25])
                + 2.0 * (wedge[e43] * wedge[e35])
                + 2.0 * (wedge[e1234] * wedge[e3215])
                + f32::powi(wedge[e23], 2)
                + f32::powi(wedge[e31], 2)
                + f32::powi(wedge[e12], 2)
                - f32::powi(wedge[e45], 2)
                - f32::powi(wedge[e4235], 2)
                - f32::powi(wedge[e4315], 2)
                - f32::powi(wedge[e4125], 2),
        );
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for MultiVector {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       30        0
    //    simd3        2        7        0
    //    simd4        5        3        0
    // Totals...
    // yes simd       38       40        0
    //  no simd       57       63        0
    fn flat_bulk_norm_squared(self) -> Scalar {
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
        return Scalar::from_groups(
            // scalar
            2.0 * (wedge[e15] * wedge[e41])
                + 2.0 * (wedge[e25] * wedge[e42])
                + 2.0 * (wedge[e35] * wedge[e43])
                + 2.0 * (wedge[e3215] * wedge[e1234])
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
        );
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for Plane {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powi(self[e3215], 2) * -1.0);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for RoundPoint {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for RoundPoint {
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powi(self[e5], 2));
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for Sphere {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powi(self[e3215], 2) * -1.0);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for VersorEven {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       14        0
    //    simd3        2        8        0
    // Totals...
    // yes simd       15       22        0
    //  no simd       19       38        0
    fn flat_bulk_norm_squared(self) -> Scalar {
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
        return Scalar::from_groups(
            // scalar
            2.0 * (wedge[e41] * wedge[e15])
                + 2.0 * (wedge[e42] * wedge[e25])
                + 2.0 * (wedge[e43] * wedge[e35])
                + 2.0 * (wedge[e1234] * wedge[e3215])
                + f32::powi(wedge[e23], 2)
                + f32::powi(wedge[e31], 2)
                + f32::powi(wedge[e12], 2)
                - f32::powi(wedge[e45], 2)
                - f32::powi(wedge[e4235], 2)
                - f32::powi(wedge[e4315], 2)
                - f32::powi(wedge[e4125], 2),
        );
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for VersorOdd {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        7       12        0
    //  no simd       12       19        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type = Flector::from_groups(
            // e15, e25, e35, e45
            self.group2().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
        );
        let other = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        let wedge = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                sub_type[e45] * other[e1] * -1.0,
                sub_type[e45] * other[e2] * -1.0,
                sub_type[e45] * other[e3] * -1.0,
                (sub_type[e4315] * other[e2]) + (sub_type[e4125] * other[e3]) + (sub_type[e3215] * other[e4]),
            ]) + (other.group0().wwwx() * sub_type.group0().xyz().with_w(sub_type[e4235])),
            // e235, e315, e125, e5
            ((sub_type.group0().zxy() * other.group0().yzx()) - (sub_type.group0().yzx() * other.group0().zxy())).with_w(0.0),
        );
        return Scalar::from_groups(
            // scalar
            -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2) - f32::powi(wedge[e12345], 2),
        );
    }
}
