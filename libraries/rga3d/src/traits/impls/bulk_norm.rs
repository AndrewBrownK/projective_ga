// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 2
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         3       2       0
//   Median:         7       6       0
//  Average:         5       4       0
//  Maximum:         7       6       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         3       4       0
//   Median:         7      13       0
//  Average:         5       8       0
//  Maximum:         7      13       0
impl std::ops::Div<BulkNormPrefixOrPostfix> for Motor {
    type Output = Scalar;
    fn div(self, _rhs: BulkNormPrefixOrPostfix) -> Self::Output {
        self.bulk_norm()
    }
}
impl BulkNorm for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        4        0
    fn bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = Motor::from_groups(/* e41, e42, e43, e1234 */ Simd32x4::from(0.0), /* e23, e31, e12, scalar */ self.group1());
        let other = Origin::from_groups(/* e4 */ 1.0);
        let wedge = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(sub_type[scalar] * other[e4]),
            // e423, e431, e412, e321
            (Simd32x3::from(other[e4]) * sub_type.group1().xyz()).with_w(0.0),
        );
        return Scalar::from_groups(
            // scalar
            f32::powi(wedge[e1], 2) + f32::powi(wedge[e2], 2) + f32::powi(wedge[e3], 2) + f32::powi(wedge[e321], 2),
        );
    }
}
impl std::ops::Div<BulkNormPrefixOrPostfix> for MultiVector {
    type Output = Scalar;
    fn div(self, _rhs: BulkNormPrefixOrPostfix) -> Self::Output {
        self.bulk_norm()
    }
}
impl BulkNorm for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        2        0
    //    simd2        0        1        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        7        6        0
    //  no simd        7       13        0
    fn bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(self[e321]),
        );
        let other = Origin::from_groups(/* e4 */ 1.0);
        let wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([1.0, sub_type[e321] * other[e4]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(sub_type[scalar] * other[e4]),
            // e41, e42, e43
            Simd32x3::from(other[e4]) * sub_type.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(other[e4]) * sub_type.group3()).with_w(0.0),
        );
        return Scalar::from_groups(
            // scalar
            f32::powi(wedge[scalar], 2)
                + f32::powi(wedge[e1], 2)
                + f32::powi(wedge[e2], 2)
                + f32::powi(wedge[e3], 2)
                + f32::powi(wedge[e23], 2)
                + f32::powi(wedge[e31], 2)
                + f32::powi(wedge[e12], 2)
                + f32::powi(wedge[e321], 2),
        );
    }
}
