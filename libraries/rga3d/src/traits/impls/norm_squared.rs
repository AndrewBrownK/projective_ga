// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 5
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       0       0
//  Average:         5       1       0
//  Maximum:        14       6       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       0       0
//  Average:         5       3       0
//  Maximum:        14      13       0
impl std::ops::Div<NormSquaredPrefixOrPostfix> for Flector {
    type Output = AntiScalar;
    fn div(self, _rhs: NormSquaredPrefixOrPostfix) -> Self::Output {
        self.norm_squared()
    }
}
impl NormSquared for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn norm_squared(self) -> Self::Output {
        use crate::elements::*;
        let sub_type_2 = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e423, e431, e412, e321
            self.group1().xyz().with_w(0.0),
        );
        return AntiScalar::from_groups(
            // e1234
            f32::powi(sub_type_2[e4], 2) + f32::powi(sub_type_2[e423], 2) + f32::powi(sub_type_2[e431], 2) + f32::powi(sub_type_2[e412], 2),
        );
    }
}
impl std::ops::Div<NormSquaredPrefixOrPostfix> for Line {
    type Output = DualNum;
    fn div(self, _rhs: NormSquaredPrefixOrPostfix) -> Self::Output {
        self.norm_squared()
    }
}
impl NormSquared for Line {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn norm_squared(self) -> Self::Output {
        use crate::elements::*;
        let sub_type_2 = Line::from_groups(/* e41, e42, e43 */ self.group0(), /* e23, e31, e12 */ Simd32x3::from(0.0));
        return DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, f32::powi(sub_type_2[e41], 2) + f32::powi(sub_type_2[e42], 2) + f32::powi(sub_type_2[e43], 2)]),
        );
    }
}
impl std::ops::Div<NormSquaredPrefixOrPostfix> for Motor {
    type Output = DualNum;
    fn div(self, _rhs: NormSquaredPrefixOrPostfix) -> Self::Output {
        self.norm_squared()
    }
}
impl NormSquared for Motor {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        6        2        0
    //  no simd        6        4        0
    fn norm_squared(self) -> Self::Output {
        use crate::elements::*;
        let sub_type = Motor::from_groups(/* e41, e42, e43, e1234 */ Simd32x4::from(0.0), /* e23, e31, e12, scalar */ self.group1());
        let other = Origin::from_groups(/* e4 */ 1.0);
        let wedge = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(sub_type[scalar] * other[e4]),
            // e423, e431, e412, e321
            (Simd32x3::from(other[e4]) * sub_type.group1().xyz()).with_w(0.0),
        );
        let sub_type_2 = Motor::from_groups(/* e41, e42, e43, e1234 */ self.group0(), /* e23, e31, e12, scalar */ Simd32x4::from(0.0));
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            f32::powi(wedge[e1], 2) + f32::powi(wedge[e2], 2) + f32::powi(wedge[e3], 2) + f32::powi(wedge[e321], 2),
            f32::powi(sub_type_2[e41], 2) + f32::powi(sub_type_2[e42], 2) + f32::powi(sub_type_2[e43], 2) + f32::powi(sub_type_2[e1234], 2),
        ]));
    }
}
impl std::ops::Div<NormSquaredPrefixOrPostfix> for MultiVector {
    type Output = DualNum;
    fn div(self, _rhs: NormSquaredPrefixOrPostfix) -> Self::Output {
        self.norm_squared()
    }
}
impl NormSquared for MultiVector {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14        2        0
    //    simd2        0        1        0
    //    simd3        0        3        0
    // Totals...
    // yes simd       14        6        0
    //  no simd       14       13        0
    fn norm_squared(self) -> Self::Output {
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
        let sub_type_2 = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, self[e1234]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            self.group4().xyz().with_w(0.0),
        );
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            f32::powi(wedge[scalar], 2)
                + f32::powi(wedge[e1], 2)
                + f32::powi(wedge[e2], 2)
                + f32::powi(wedge[e3], 2)
                + f32::powi(wedge[e23], 2)
                + f32::powi(wedge[e31], 2)
                + f32::powi(wedge[e12], 2)
                + f32::powi(wedge[e321], 2),
            f32::powi(sub_type_2[e1234], 2)
                + f32::powi(sub_type_2[e4], 2)
                + f32::powi(sub_type_2[e41], 2)
                + f32::powi(sub_type_2[e42], 2)
                + f32::powi(sub_type_2[e43], 2)
                + f32::powi(sub_type_2[e423], 2)
                + f32::powi(sub_type_2[e431], 2)
                + f32::powi(sub_type_2[e412], 2),
        ]));
    }
}
impl std::ops::Div<NormSquaredPrefixOrPostfix> for Point {
    type Output = AntiScalar;
    fn div(self, _rhs: NormSquaredPrefixOrPostfix) -> Self::Output {
        self.norm_squared()
    }
}
impl NormSquared for Point {
    type Output = AntiScalar;
    fn norm_squared(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ f32::powi(self[e4], 2));
    }
}
