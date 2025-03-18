// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 5
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       0       0
//  Average:         3       0       0
//  Maximum:         7       0       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       0       0
//  Average:         3       0       0
//  Maximum:         7       0       0
impl std::ops::Div<NormPrefixOrPostfix> for Flector {
    type Output = AntiScalar;
    fn div(self, _rhs: NormPrefixOrPostfix) -> Self::Output {
        self.norm()
    }
}
impl Norm for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type_g1_xyz = self.group1().xyz();
        AntiScalar::from_groups(
            // e1234
            sub_type_g1_xyz[0] * sub_type_g1_xyz[0] + sub_type_g1_xyz[1] * sub_type_g1_xyz[1] + sub_type_g1_xyz[2] * sub_type_g1_xyz[2] + self[e4] * self[e4],
        )
    }
}
impl std::ops::Div<NormPrefixOrPostfix> for Line {
    type Output = AntiScalar;
    fn div(self, _rhs: NormPrefixOrPostfix) -> Self::Output {
        self.norm()
    }
}
impl Norm for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43])
    }
}
impl std::ops::Div<NormPrefixOrPostfix> for Motor {
    type Output = AntiScalar;
    fn div(self, _rhs: NormPrefixOrPostfix) -> Self::Output {
        self.norm()
    }
}
impl Norm for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234])
    }
}
impl std::ops::Div<NormPrefixOrPostfix> for MultiVector {
    type Output = AntiScalar;
    fn div(self, _rhs: NormPrefixOrPostfix) -> Self::Output {
        self.norm()
    }
}
impl Norm for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn norm(self) -> AntiScalar {
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
impl std::ops::Div<NormPrefixOrPostfix> for Point {
    type Output = AntiScalar;
    fn div(self, _rhs: NormPrefixOrPostfix) -> Self::Output {
        self.norm()
    }
}
impl Norm for Point {
    fn norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e4])
    }
}
