// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
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
impl std::ops::Div<WeightNormSquaredPrefixOrPostfix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: WeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.weight_norm_squared()
    }
}
impl std::ops::DivAssign<WeightNormSquaredPrefixOrPostfix> for AntiScalar {
    fn div_assign(&mut self, _rhs: WeightNormSquaredPrefixOrPostfix) {
        *self = self.weight_norm_squared()
    }
}
impl WeightNormSquared for AntiScalar {
    fn weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * self[e1234])
    }
}
impl std::ops::Div<WeightNormSquaredPrefixOrPostfix> for DualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: WeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.weight_norm_squared()
    }
}
impl WeightNormSquared for DualNum {
    fn weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * self[e1234])
    }
}
impl std::ops::Div<WeightNormSquaredPrefixOrPostfix> for Flector {
    type Output = AntiScalar;
    fn div(self, _rhs: WeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.weight_norm_squared()
    }
}
impl WeightNormSquared for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type_g1_xyz = self.group1().xyz();
        AntiScalar::from_groups(
            // e1234
            sub_type_g1_xyz[0] * sub_type_g1_xyz[0] + sub_type_g1_xyz[1] * sub_type_g1_xyz[1] + sub_type_g1_xyz[2] * sub_type_g1_xyz[2] + self[e4] * self[e4],
        )
    }
}
impl std::ops::Div<WeightNormSquaredPrefixOrPostfix> for Line {
    type Output = AntiScalar;
    fn div(self, _rhs: WeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.weight_norm_squared()
    }
}
impl WeightNormSquared for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43])
    }
}
impl std::ops::Div<WeightNormSquaredPrefixOrPostfix> for Motor {
    type Output = AntiScalar;
    fn div(self, _rhs: WeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.weight_norm_squared()
    }
}
impl WeightNormSquared for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234])
    }
}
impl std::ops::Div<WeightNormSquaredPrefixOrPostfix> for MultiVector {
    type Output = AntiScalar;
    fn div(self, _rhs: WeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.weight_norm_squared()
    }
}
impl WeightNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type_g4_xyz = self.group4().xyz();
        AntiScalar::from_groups(
            // e1234
            sub_type_g4_xyz[0] * sub_type_g4_xyz[0]
                + sub_type_g4_xyz[1] * sub_type_g4_xyz[1]
                + sub_type_g4_xyz[2] * sub_type_g4_xyz[2]
                + self[e1234] * self[e1234]
                + self[e4] * self[e4]
                + self[e41] * self[e41]
                + self[e42] * self[e42]
                + self[e43] * self[e43],
        )
    }
}
impl std::ops::Div<WeightNormSquaredPrefixOrPostfix> for Origin {
    type Output = AntiScalar;
    fn div(self, _rhs: WeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.weight_norm_squared()
    }
}
impl WeightNormSquared for Origin {
    fn weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e4] * self[e4])
    }
}
impl std::ops::Div<WeightNormSquaredPrefixOrPostfix> for Plane {
    type Output = AntiScalar;
    fn div(self, _rhs: WeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.weight_norm_squared()
    }
}
impl WeightNormSquared for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn weight_norm_squared(self) -> AntiScalar {
        let sub_type_g0_xyz = self.group0().xyz();
        AntiScalar::from_groups(
            // e1234
            sub_type_g0_xyz[0] * sub_type_g0_xyz[0] + sub_type_g0_xyz[1] * sub_type_g0_xyz[1] + sub_type_g0_xyz[2] * sub_type_g0_xyz[2],
        )
    }
}
impl std::ops::Div<WeightNormSquaredPrefixOrPostfix> for Point {
    type Output = AntiScalar;
    fn div(self, _rhs: WeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.weight_norm_squared()
    }
}
impl WeightNormSquared for Point {
    fn weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e4] * self[e4])
    }
}
