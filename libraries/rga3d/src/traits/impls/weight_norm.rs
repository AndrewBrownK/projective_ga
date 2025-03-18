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
        self
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
        AntiScalar::from_groups(/* e1234 */ self[e1234])
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
        let sub_type_g1_xyz = self.group1().xyz();
        AntiScalar::from_groups(
            // e1234
            sub_type_g1_xyz[0] * sub_type_g1_xyz[0] + sub_type_g1_xyz[1] * sub_type_g1_xyz[1] + sub_type_g1_xyz[2] * sub_type_g1_xyz[2] + self[e4] * self[e4],
        )
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
        AntiScalar::from_groups(/* e1234 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43])
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
        AntiScalar::from_groups(/* e1234 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234])
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
impl std::ops::Div<WeightNormPrefixOrPostfix> for Origin {
    type Output = AntiScalar;
    fn div(self, _rhs: WeightNormPrefixOrPostfix) -> Self::Output {
        self.weight_norm()
    }
}
impl WeightNorm for Origin {
    fn weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e4])
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
        let sub_type_g0_xyz = self.group0().xyz();
        AntiScalar::from_groups(
            // e1234
            sub_type_g0_xyz[0] * sub_type_g0_xyz[0] + sub_type_g0_xyz[1] * sub_type_g0_xyz[1] + sub_type_g0_xyz[2] * sub_type_g0_xyz[2],
        )
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
        AntiScalar::from_groups(/* e1234 */ self[e4])
    }
}
