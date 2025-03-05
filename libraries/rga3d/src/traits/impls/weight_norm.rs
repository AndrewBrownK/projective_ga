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
//   Median:         2       0       0
//  Average:         1       0       0
//  Maximum:         7       0       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       0       0
//  Average:         1       0       0
//  Maximum:         7       0       0
impl std::ops::Div<WeightNormPrefixOrPostfix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: WeightNormPrefixOrPostfix) -> Self::Output {
        self.weight_norm()
    }
}
impl std::ops::DivAssign<WeightNormPrefixOrPostfix> for AntiScalar {
    fn div_assign(&mut self, _rhs: WeightNormPrefixOrPostfix) {
        *self = self.weight_norm()
    }
}
impl WeightNorm for AntiScalar {
    fn weight_norm(self) -> AntiScalar {
        return self;
    }
}
impl std::ops::Div<WeightNormPrefixOrPostfix> for DualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: WeightNormPrefixOrPostfix) -> Self::Output {
        self.weight_norm()
    }
}
impl WeightNorm for DualNum {
    fn weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[e1234]);
    }
}
impl std::ops::Div<WeightNormPrefixOrPostfix> for Flector {
    type Output = AntiScalar;
    fn div(self, _rhs: WeightNormPrefixOrPostfix) -> Self::Output {
        self.weight_norm()
    }
}
impl WeightNorm for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e423, e431, e412, e321
            self.group1().xyz().with_w(0.0),
        );
        return AntiScalar::from_groups(
            // e1234
            f32::powi(sub_type[e4], 2) + f32::powi(sub_type[e423], 2) + f32::powi(sub_type[e431], 2) + f32::powi(sub_type[e412], 2),
        );
    }
}
impl std::ops::Div<WeightNormPrefixOrPostfix> for Line {
    type Output = AntiScalar;
    fn div(self, _rhs: WeightNormPrefixOrPostfix) -> Self::Output {
        self.weight_norm()
    }
}
impl WeightNorm for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = Line::from_groups(/* e41, e42, e43 */ self.group0(), /* e23, e31, e12 */ Simd32x3::from(0.0));
        return AntiScalar::from_groups(/* e1234 */ f32::powi(sub_type[e41], 2) + f32::powi(sub_type[e42], 2) + f32::powi(sub_type[e43], 2));
    }
}
impl std::ops::Div<WeightNormPrefixOrPostfix> for Motor {
    type Output = AntiScalar;
    fn div(self, _rhs: WeightNormPrefixOrPostfix) -> Self::Output {
        self.weight_norm()
    }
}
impl WeightNorm for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = Motor::from_groups(/* e41, e42, e43, e1234 */ self.group0(), /* e23, e31, e12, scalar */ Simd32x4::from(0.0));
        return AntiScalar::from_groups(
            // e1234
            f32::powi(sub_type[e41], 2) + f32::powi(sub_type[e42], 2) + f32::powi(sub_type[e43], 2) + f32::powi(sub_type[e1234], 2),
        );
    }
}
impl std::ops::Div<WeightNormPrefixOrPostfix> for MultiVector {
    type Output = AntiScalar;
    fn div(self, _rhs: WeightNormPrefixOrPostfix) -> Self::Output {
        self.weight_norm()
    }
}
impl WeightNorm for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = MultiVector::from_groups(
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
        return AntiScalar::from_groups(
            // e1234
            f32::powi(sub_type[e1234], 2)
                + f32::powi(sub_type[e4], 2)
                + f32::powi(sub_type[e41], 2)
                + f32::powi(sub_type[e42], 2)
                + f32::powi(sub_type[e43], 2)
                + f32::powi(sub_type[e423], 2)
                + f32::powi(sub_type[e431], 2)
                + f32::powi(sub_type[e412], 2),
        );
    }
}
impl std::ops::Div<WeightNormPrefixOrPostfix> for Origin {
    type Output = AntiScalar;
    fn div(self, _rhs: WeightNormPrefixOrPostfix) -> Self::Output {
        self.weight_norm()
    }
}
impl WeightNorm for Origin {
    fn weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[e4]);
    }
}
impl std::ops::Div<WeightNormPrefixOrPostfix> for Plane {
    type Output = AntiScalar;
    fn div(self, _rhs: WeightNormPrefixOrPostfix) -> Self::Output {
        self.weight_norm()
    }
}
impl WeightNorm for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = Plane::from_groups(/* e423, e431, e412, e321 */ self.group0().xyz().with_w(0.0));
        return AntiScalar::from_groups(/* e1234 */ f32::powi(sub_type[e423], 2) + f32::powi(sub_type[e431], 2) + f32::powi(sub_type[e412], 2));
    }
}
impl std::ops::Div<WeightNormPrefixOrPostfix> for Point {
    type Output = AntiScalar;
    fn div(self, _rhs: WeightNormPrefixOrPostfix) -> Self::Output {
        self.weight_norm()
    }
}
impl WeightNorm for Point {
    fn weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[e4]);
    }
}
