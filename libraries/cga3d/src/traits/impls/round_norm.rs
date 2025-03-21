// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 10
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         2       3       0     N/A
//   Median:         5       7       0     N/A
//  Average:         5       6       0     N/A
//  Maximum:        14      16       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         2       3       0       0
//   Median:         5       7       0       0
//  Average:         5       6       0       0
//  Maximum:        14      16       0       0
impl std::ops::Div<RoundNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        7        0        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        let sub_type_g0_xyz = self.group1().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                sub_type_g0_xyz[0] * sub_type_g0_xyz[0] + sub_type_g0_xyz[1] * sub_type_g0_xyz[1] + sub_type_g0_xyz[2] * sub_type_g0_xyz[2] + self[scalar] * self[scalar],
                self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Div<RoundNormPrefixOrPostfix> for AntiDipoleInversion {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        6        8        0        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        let sub_type_g1_xyz = self.group3().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                sub_type_g1_xyz[0] * sub_type_g1_xyz[0] + sub_type_g1_xyz[1] * sub_type_g1_xyz[1] + sub_type_g1_xyz[2] * sub_type_g1_xyz[2] + self[e321] * self[e321],
                self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Div<RoundNormPrefixOrPostfix> for Circle {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[e321], self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Div<RoundNormPrefixOrPostfix> for CircleRotor {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[e321], self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Div<RoundNormPrefixOrPostfix> for Dipole {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4        6        0        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        let sub_type_g0 = self.group1().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                sub_type_g0[0] * sub_type_g0[0] + sub_type_g0[1] * sub_type_g0[1] + sub_type_g0[2] * sub_type_g0[2],
                self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Div<RoundNormPrefixOrPostfix> for DipoleInversion {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        7        0        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        let sub_type_g0 = self.group1().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                sub_type_g0[0] * sub_type_g0[0] + sub_type_g0[1] * sub_type_g0[1] + sub_type_g0[2] * sub_type_g0[2],
                self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Div<RoundNormPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl std::ops::DivAssign<RoundNormPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: RoundNormPrefixOrPostfix) {
        *self = self.round_norm()
    }
}
impl RoundNorm for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32       14       16        0        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        let sub_type_g1_xyz = self.group1().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                sub_type_g1_xyz[0] * sub_type_g1_xyz[0]
                    + sub_type_g1_xyz[1] * sub_type_g1_xyz[1]
                    + sub_type_g1_xyz[2] * sub_type_g1_xyz[2]
                    + self[scalar] * self[scalar]
                    + self[e23] * self[e23]
                    + self[e31] * self[e31]
                    + self[e12] * self[e12]
                    + self[e321] * self[e321],
                self[e4] * self[e4]
                    + self[e41] * self[e41]
                    + self[e42] * self[e42]
                    + self[e43] * self[e43]
                    + self[e423] * self[e423]
                    + self[e431] * self[e431]
                    + self[e412] * self[e412]
                    + self[e1234] * self[e1234],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Div<RoundNormPrefixOrPostfix> for RoundPoint {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        let sub_type_g0_xyz = self.group0().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                sub_type_g0_xyz[0] * sub_type_g0_xyz[0] + sub_type_g0_xyz[1] * sub_type_g0_xyz[1] + sub_type_g0_xyz[2] * sub_type_g0_xyz[2],
                self[e4],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Div<RoundNormPrefixOrPostfix> for VersorEven {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        6        8        0        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        let sub_type_g1_xyz = self.group3().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                sub_type_g1_xyz[0] * sub_type_g1_xyz[0] + sub_type_g1_xyz[1] * sub_type_g1_xyz[1] + sub_type_g1_xyz[2] * sub_type_g1_xyz[2] + self[e321] * self[e321],
                self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Div<RoundNormPrefixOrPostfix> for VersorOdd {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        6        8        0        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        let sub_type_g0 = self.group0().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self[scalar] * self[scalar] + self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12],
                sub_type_g0[0] * sub_type_g0[0] + sub_type_g0[1] * sub_type_g0[1] + sub_type_g0[2] * sub_type_g0[2] + self[e1234] * self[e1234],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
