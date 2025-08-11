// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 17
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         2       3       0     N/A
//  Average:         2       2       0     N/A
//  Maximum:         7       8       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         2       3       0       0
//  Average:         2       2       0       0
//  Maximum:         7       8       0       0
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for AntiCircleRotor {
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e45])
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
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        let sub_type_g0 = self.group1().xyz();
        AntiScalar::from_groups(/* e12345 */ sub_type_g0[0] * sub_type_g0[0] + sub_type_g0[1] * sub_type_g0[1] + sub_type_g0[2] * sub_type_g0[2])
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
        self
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
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        let sub_type_g0 = self.group1().xyz();
        AntiScalar::from_groups(/* e12345 */ sub_type_g0[0] * sub_type_g0[0] + sub_type_g0[1] * sub_type_g0[1] + sub_type_g0[2] * sub_type_g0[2])
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
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type_g0_xyz = self.group1().xyz();
        AntiScalar::from_groups(
            // e12345
            sub_type_g0_xyz[0] * sub_type_g0_xyz[0] + sub_type_g0_xyz[1] * sub_type_g0_xyz[1] + sub_type_g0_xyz[2] * sub_type_g0_xyz[2] + self[e12345] * self[e12345],
        )
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
        AntiScalar::from_groups(/* e12345 */ self[e45])
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
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type_g1_xyz = self.group3().xyz();
        AntiScalar::from_groups(
            // e12345
            sub_type_g1_xyz[0] * sub_type_g1_xyz[0] + sub_type_g1_xyz[1] * sub_type_g1_xyz[1] + sub_type_g1_xyz[2] * sub_type_g1_xyz[2] + self[e45] * self[e45],
        )
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
        AntiScalar::from_groups(/* e12345 */ self[e12345])
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
        AntiScalar::from_groups(/* e12345 */ self[e45])
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
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type_g1_xyz = self.group1().xyz();
        AntiScalar::from_groups(
            // e12345
            sub_type_g1_xyz[0] * sub_type_g1_xyz[0] + sub_type_g1_xyz[1] * sub_type_g1_xyz[1] + sub_type_g1_xyz[2] * sub_type_g1_xyz[2] + self[e45] * self[e45],
        )
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
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435])
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
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] + self[e12345] * self[e12345],
        )
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
    //      add/sub      mul      div      pow
    // f32        7        8        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type_g6_xyz = self.group6().xyz();
        let sub_type_g9_xyz = self.group9().xyz();
        AntiScalar::from_groups(
            // e12345
            self[e12345] * self[e12345]
                + sub_type_g6_xyz[0] * sub_type_g6_xyz[0]
                + sub_type_g6_xyz[1] * sub_type_g6_xyz[1]
                + sub_type_g6_xyz[2] * sub_type_g6_xyz[2]
                + sub_type_g9_xyz[0] * sub_type_g9_xyz[0]
                + sub_type_g9_xyz[1] * sub_type_g9_xyz[1]
                + sub_type_g9_xyz[2] * sub_type_g9_xyz[2]
                + self[e45] * self[e45],
        )
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
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        let sub_type_g0_xyz = self.group0().xyz();
        AntiScalar::from_groups(
            // e12345
            sub_type_g0_xyz[0] * sub_type_g0_xyz[0] + sub_type_g0_xyz[1] * sub_type_g0_xyz[1] + sub_type_g0_xyz[2] * sub_type_g0_xyz[2],
        )
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
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        let sub_type_g0_xyz = self.group0().xyz();
        AntiScalar::from_groups(
            // e12345
            sub_type_g0_xyz[0] * sub_type_g0_xyz[0] + sub_type_g0_xyz[1] * sub_type_g0_xyz[1] + sub_type_g0_xyz[2] * sub_type_g0_xyz[2],
        )
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
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type_g0_xyz = self.group1().xyz();
        AntiScalar::from_groups(
            // e12345
            sub_type_g0_xyz[0] * sub_type_g0_xyz[0] + sub_type_g0_xyz[1] * sub_type_g0_xyz[1] + sub_type_g0_xyz[2] * sub_type_g0_xyz[2] + self[e12345] * self[e12345],
        )
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
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type_g1_xyz = self.group3().xyz();
        AntiScalar::from_groups(
            // e12345
            sub_type_g1_xyz[0] * sub_type_g1_xyz[0] + sub_type_g1_xyz[1] * sub_type_g1_xyz[1] + sub_type_g1_xyz[2] * sub_type_g1_xyz[2] + self[e45] * self[e45],
        )
    }
}
