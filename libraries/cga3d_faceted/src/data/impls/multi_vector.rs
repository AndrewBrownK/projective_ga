use crate::traits::GeometricProduct;
use crate::traits::RightDual;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 381
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       0       0
//  Average:        21      27       0
//  Maximum:       404     459       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         7       0       0
//  Average:        50      55       0
//  Maximum:       992    1024       0
impl std::ops::Add<AntiCircleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: AntiCircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0() + self.group3().xyz()).with_w(self[e45]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            other.group1() + self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiCircleOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiCircleOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0() + self.group3().xyz()).with_w(self[e45]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            other.group1() + self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        1        0        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       12        0        0
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + other.group0().with_w(other[e45]),
            // e15, e25, e35
            self.group4() + other.group2().xyz(),
            // e23, e31, e12
            self.group5() + other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiCircleRotor> for MultiVector {
    fn add_assign(&mut self, other: AntiCircleRotor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + other.group0().with_w(other[e45]),
            // e15, e25, e35
            self.group4() + other.group2().xyz(),
            // e23, e31, e12
            self.group5() + other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        1        0        0
    //    simd3        3        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       11        0        0
    fn add(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0() + self.group3().xyz()).with_w(self[e45]),
            // e15, e25, e35
            self.group4() + other.group2().xyz(),
            // e23, e31, e12
            other.group1() + self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiCircleRotorAligningOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiCircleRotorAligningOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0() + self.group3().xyz()).with_w(self[e45]),
            // e15, e25, e35
            self.group4() + other.group2().xyz(),
            // e23, e31, e12
            other.group1() + self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOriginAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        1        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        8        0        0
    fn add(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4() + other.group1().xyz(),
            // e23, e31, e12
            other.group0() + self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiCircleRotorAligningOriginAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4() + other.group1().xyz(),
            // e23, e31, e12
            other.group0() + self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiCircleRotorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd2        1        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        9        0        0
    fn add(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3().xyz().with_w(other[e45] + self[e45]),
            // e15, e25, e35
            self.group4() + other.group1().xyz(),
            // e23, e31, e12
            self.group5() + other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiCircleRotorAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: AntiCircleRotorAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3().xyz().with_w(other[e45] + self[e45]),
            // e15, e25, e35
            self.group4() + other.group1().xyz(),
            // e23, e31, e12
            self.group5() + other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiCircleRotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        1        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        8        0        0
    fn add(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() + self.group3().xyz()).with_w(self[e45]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            other.group1() + self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiCircleRotorOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiCircleRotorOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() + self.group3().xyz()).with_w(self[e45]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            other.group1() + self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       15        0        0
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + other.group3().xyz().with_w(other[e4]),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            other.group1() + self.group6(),
            // e423, e431, e412
            other.group0() + self.group7(),
            // e235, e315, e125
            self.group8() + other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiDipoleInversion> for MultiVector {
    fn add_assign(&mut self, other: AntiDipoleInversion) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + other.group3().xyz().with_w(other[e4]),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            other.group1() + self.group6(),
            // e423, e431, e412
            other.group0() + self.group7(),
            // e235, e315, e125
            self.group8() + other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDipoleInversionAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       11        0        0
    fn add(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (other.group2().xyz() + self.group1().xyz()).with_w(self[e4]),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            other.group0() + self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            other.group1() + self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiDipoleInversionAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: AntiDipoleInversionAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (other.group2().xyz() + self.group1().xyz()).with_w(self[e4]),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            other.group0() + self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            other.group1() + self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        8        0        0
    fn add(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + other.group1().yzwx(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6().xyz().with_w(other[e321] + self[e321]),
            // e423, e431, e412
            self.group7() + other.group0().xyz(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiDipoleInversionOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiDipoleInversionOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + other.group1().yzwx(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6().xyz().with_w(other[e321] + self[e321]),
            // e423, e431, e412
            self.group7() + other.group0().xyz(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        3        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       11        0        0
    fn add(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(other[e4] + self[e4]),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (other.group1() + self.group6().xyz()).with_w(self[e321]),
            // e423, e431, e412
            self.group7() + other.group0().xyz(),
            // e235, e315, e125
            self.group8() + other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiDipoleInversionOrthogonalOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiDipoleInversionOrthogonalOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(other[e4] + self[e4]),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (other.group1() + self.group6().xyz()).with_w(self[e321]),
            // e423, e431, e412
            self.group7() + other.group0().xyz(),
            // e235, e315, e125
            self.group8() + other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDipoleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: AntiDipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6().xyz().with_w(other[e321] + self[e321]),
            // e423, e431, e412
            self.group7() + other.group0().xyz(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiDipoleOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiDipoleOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6().xyz().with_w(other[e321] + self[e321]),
            // e423, e431, e412
            self.group7() + other.group0().xyz(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        6        0        0
    fn add(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiDualNum> for MultiVector {
    fn add_assign(&mut self, other: AntiDualNum) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiFlatOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiFlatOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiFlatOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiFlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6().xyz().with_w(other[e321] + self[e321]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() + other.group0().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiFlatPoint> for MultiVector {
    fn add_assign(&mut self, other: AntiFlatPoint) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6().xyz().with_w(other[e321] + self[e321]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() + other.group0().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiFlector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        8        0        0
    fn add(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (other.group1().xyz() + self.group1().xyz()).with_w(self[e4]),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6().xyz().with_w(other[e321] + self[e321]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() + other.group0().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiFlector> for MultiVector {
    fn add_assign(&mut self, other: AntiFlector) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (other.group1().xyz() + self.group1().xyz()).with_w(self[e4]),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6().xyz().with_w(other[e321] + self[e321]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() + other.group0().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiFlectorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (self.group1().xyz() + other.group0().yzw()).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6().xyz().with_w(other[e321] + self[e321]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiFlectorOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiFlectorOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (self.group1().xyz() + other.group0().yzw()).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6().xyz().with_w(other[e321] + self[e321]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiLine> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            other.group1() + self.group4(),
            // e23, e31, e12
            other.group0() + self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiLine> for MultiVector {
    fn add_assign(&mut self, other: AntiLine) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            other.group1() + self.group4(),
            // e23, e31, e12
            other.group0() + self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiLineOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: AntiLineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            other.group0() + self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiLineOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiLineOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            other.group0() + self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiMotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd2        1        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        9        0        0
    fn add(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4() + other.group1().xyz(),
            // e23, e31, e12
            self.group5() + other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiMotor> for MultiVector {
    fn add_assign(&mut self, other: AntiMotor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4() + other.group1().xyz(),
            // e23, e31, e12
            self.group5() + other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::Add<AntiMotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5() + other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiMotorOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiMotorOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5() + other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiMysteryCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd2        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        6        0        0
    fn add(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3().xyz().with_w(other[e45] + self[e45]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5() + other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiMysteryCircleRotor> for MultiVector {
    fn add_assign(&mut self, other: AntiMysteryCircleRotor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3().xyz().with_w(other[e45] + self[e45]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5() + other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiMysteryDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (other.group1() + self.group1().xyz()).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            other.group0() + self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiMysteryDipoleInversion> for MultiVector {
    fn add_assign(&mut self, other: AntiMysteryDipoleInversion) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (other.group1() + self.group1().xyz()).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            other.group0() + self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiPlane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (other.group0().xyz() + self.group1().xyz()).with_w(self[e4]),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiPlane> for MultiVector {
    fn add_assign(&mut self, other: AntiPlane) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (other.group0().xyz() + self.group1().xyz()).with_w(self[e4]),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiPlaneOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: AntiPlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (other.group0() + self.group1().xyz()).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiPlaneOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiPlaneOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (other.group0() + self.group1().xyz()).with_w(self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        1        0        0
    // no simd        2        0        0
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiScalar> for MultiVector {
    fn add_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiSphereOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: AntiSphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            other.group0() + self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiSphereOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiSphereOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            other.group0() + self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiVersorEvenOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        1        0        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       12        0        0
    fn add(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() + self.group3().xyz()).with_w(self[e45]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5() + other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiVersorEvenOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiVersorEvenOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() + self.group3().xyz()).with_w(self[e45]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5() + other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Circle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn add(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            other.group1() + self.group6(),
            // e423, e431, e412
            other.group0() + self.group7(),
            // e235, e315, e125
            other.group2() + self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<Circle> for MultiVector {
    fn add_assign(&mut self, other: Circle) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            other.group1() + self.group6(),
            // e423, e431, e412
            other.group0() + self.group7(),
            // e235, e315, e125
            other.group2() + self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        3        0        0
    // no simd        9        0        0
    fn add(self, other: CircleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (other.group1() + self.group6().xyz()).with_w(self[e321]),
            // e423, e431, e412
            other.group0() + self.group7(),
            // e235, e315, e125
            other.group2() + self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<CircleAligningOrigin> for MultiVector {
    fn add_assign(&mut self, other: CircleAligningOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (other.group1() + self.group6().xyz()).with_w(self[e321]),
            // e423, e431, e412
            other.group0() + self.group7(),
            // e235, e315, e125
            other.group2() + self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: CircleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            other.group0() + self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            other.group1() + self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<CircleAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: CircleAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            other.group0() + self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            other.group1() + self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: CircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            other.group0() + self.group7(),
            // e235, e315, e125
            other.group1() + self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<CircleAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: CircleAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            other.group0() + self.group7(),
            // e235, e315, e125
            other.group1() + self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: CircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (other.group1() + self.group6().xyz()).with_w(self[e321]),
            // e423, e431, e412
            other.group0() + self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<CircleOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: CircleOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (other.group1() + self.group6().xyz()).with_w(self[e321]),
            // e423, e431, e412
            other.group0() + self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        7        0        0
    fn add(self, other: CircleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6().xyz().with_w(other[e321] + self[e321]),
            // e423, e431, e412
            self.group7() + other.group0().xyz(),
            // e235, e315, e125
            other.group1() + self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<CircleOrthogonalOrigin> for MultiVector {
    fn add_assign(&mut self, other: CircleOrthogonalOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6().xyz().with_w(other[e321] + self[e321]),
            // e423, e431, e412
            self.group7() + other.group0().xyz(),
            // e235, e315, e125
            other.group1() + self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        1        0        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       12        0        0
    fn add(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            other.group1() + self.group6(),
            // e423, e431, e412
            other.group0() + self.group7(),
            // e235, e315, e125
            self.group8() + other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<CircleRotor> for MultiVector {
    fn add_assign(&mut self, other: CircleRotor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            other.group1() + self.group6(),
            // e423, e431, e412
            other.group0() + self.group7(),
            // e235, e315, e125
            self.group8() + other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotorAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        1        0        0
    //    simd3        3        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       11        0        0
    fn add(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (other.group1() + self.group6().xyz()).with_w(self[e321]),
            // e423, e431, e412
            other.group0() + self.group7(),
            // e235, e315, e125
            self.group8() + other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<CircleRotorAligningOrigin> for MultiVector {
    fn add_assign(&mut self, other: CircleRotorAligningOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (other.group1() + self.group6().xyz()).with_w(self[e321]),
            // e423, e431, e412
            other.group0() + self.group7(),
            // e235, e315, e125
            self.group8() + other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotorAligningOriginAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        1        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        8        0        0
    fn add(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (other.group0() + self.group6().xyz()).with_w(self[e321]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() + other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<CircleRotorAligningOriginAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: CircleRotorAligningOriginAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (other.group0() + self.group6().xyz()).with_w(self[e321]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() + other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        9        0        0
    fn add(self, other: CircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            other.group0() + self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() + other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<CircleRotorAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: CircleRotorAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            other.group0() + self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() + other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        1        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        8        0        0
    fn add(self, other: CircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (other.group1() + self.group6().xyz()).with_w(self[e321]),
            // e423, e431, e412
            self.group7() + other.group0().xyz(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<CircleRotorOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: CircleRotorOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (other.group1() + self.group6().xyz()).with_w(self[e321]),
            // e423, e431, e412
            self.group7() + other.group0().xyz(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Dipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn add(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + other.group0().with_w(other[e45]),
            // e15, e25, e35
            other.group2() + self.group4(),
            // e23, e31, e12
            self.group5() + other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<Dipole> for MultiVector {
    fn add_assign(&mut self, other: Dipole) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + other.group0().with_w(other[e45]),
            // e15, e25, e35
            other.group2() + self.group4(),
            // e23, e31, e12
            self.group5() + other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<DipoleAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0() + self.group3(),
            // e15, e25, e35
            other.group1() + self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<DipoleAligningOrigin> for MultiVector {
    fn add_assign(&mut self, other: DipoleAligningOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0() + self.group3(),
            // e15, e25, e35
            other.group1() + self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<DipoleAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        7        0        0
    fn add(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3().xyz().with_w(other[e45] + self[e45]),
            // e15, e25, e35
            other.group1() + self.group4(),
            // e23, e31, e12
            self.group5() + other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<DipoleAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: DipoleAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3().xyz().with_w(other[e45] + self[e45]),
            // e15, e25, e35
            other.group1() + self.group4(),
            // e23, e31, e12
            self.group5() + other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<DipoleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0() + self.group3().xyz()).with_w(self[e45]),
            // e15, e25, e35
            other.group1() + self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<DipoleAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: DipoleAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0() + self.group3().xyz()).with_w(self[e45]),
            // e15, e25, e35
            other.group1() + self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       15        0        0
    fn add(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + other.group0().with_w(other[e45]),
            // e15, e25, e35
            self.group4() + other.group2().xyz(),
            // e23, e31, e12
            self.group5() + other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], self[e4315], self[e4125]]) + self.group9().xy().with_zw(other[e4315], other[e4125]),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::AddAssign<DipoleInversion> for MultiVector {
    fn add_assign(&mut self, other: DipoleInversion) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + other.group0().with_w(other[e45]),
            // e15, e25, e35
            self.group4() + other.group2().xyz(),
            // e23, e31, e12
            self.group5() + other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], self[e4315], self[e4125]]) + self.group9().xy().with_zw(other[e4315], other[e4125]),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       12        0        0
    fn add(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0() + self.group3(),
            // e15, e25, e35
            self.group4() + other.group1().xyz(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], self[e4315], self[e4125]]) + self.group9().xy().with_zw(other[e4315], other[e4125]),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::AddAssign<DipoleInversionAligningOrigin> for MultiVector {
    fn add_assign(&mut self, other: DipoleInversionAligningOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0() + self.group3(),
            // e15, e25, e35
            self.group4() + other.group1().xyz(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], self[e4315], self[e4125]]) + self.group9().xy().with_zw(other[e4315], other[e4125]),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       12        0        0
    fn add(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3().xyz().with_w(other[e45] + self[e45]),
            // e15, e25, e35
            other.group1() + self.group4(),
            // e23, e31, e12
            self.group5() + other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], self[e4315], self[e4125]]) + self.group9().xy().with_zw(other[e4315], other[e4125]),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::AddAssign<DipoleInversionAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: DipoleInversionAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3().xyz().with_w(other[e45] + self[e45]),
            // e15, e25, e35
            other.group1() + self.group4(),
            // e23, e31, e12
            self.group5() + other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], self[e4315], self[e4125]]) + self.group9().xy().with_zw(other[e4315], other[e4125]),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       11        0        0
    fn add(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() + self.group3().xyz()).with_w(self[e45]),
            // e15, e25, e35
            self.group4() + other.group1().xyz(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::AddAssign<DipoleInversionAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: DipoleInversionAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() + self.group3().xyz()).with_w(self[e45]),
            // e15, e25, e35
            self.group4() + other.group1().xyz(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0() + self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            other.group1() + self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<DipoleInversionOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: DipoleInversionOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0() + self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            other.group1() + self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       14        0        0
    fn add(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() + self.group3().xyz()).with_w(self[e45]),
            // e15, e25, e35
            self.group4() + other.group2().xyz(),
            // e23, e31, e12
            other.group1() + self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::AddAssign<DipoleInversionOrthogonalOrigin> for MultiVector {
    fn add_assign(&mut self, other: DipoleInversionOrthogonalOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0().xyz() + self.group3().xyz()).with_w(self[e45]),
            // e15, e25, e35
            self.group4() + other.group2().xyz(),
            // e23, e31, e12
            other.group1() + self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::Add<DipoleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0() + self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<DipoleOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: DipoleOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0() + self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<DipoleOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        3        0        0
    // no simd        9        0        0
    fn add(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0() + self.group3().xyz()).with_w(self[e45]),
            // e15, e25, e35
            other.group2() + self.group4(),
            // e23, e31, e12
            other.group1() + self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<DipoleOrthogonalOrigin> for MultiVector {
    fn add_assign(&mut self, other: DipoleOrthogonalOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0() + self.group3().xyz()).with_w(self[e45]),
            // e15, e25, e35
            other.group2() + self.group4(),
            // e23, e31, e12
            other.group1() + self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd2        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        3        0        0
    fn add(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(other[e4] + self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<DualNum> for MultiVector {
    fn add_assign(&mut self, other: DualNum) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(other[e4] + self[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<FlatOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<FlatOrigin> for MultiVector {
    fn add_assign(&mut self, other: FlatOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<FlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3().xyz().with_w(other[e45] + self[e45]),
            // e15, e25, e35
            self.group4() + other.group0().xyz(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<FlatPoint> for MultiVector {
    fn add_assign(&mut self, other: FlatPoint) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3().xyz().with_w(other[e45] + self[e45]),
            // e15, e25, e35
            self.group4() + other.group0().xyz(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: FlatPointAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            other.group0() + self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<FlatPointAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: FlatPointAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            other.group0() + self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        9        0        0
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3().xyz().with_w(other[e45] + self[e45]),
            // e15, e25, e35
            self.group4() + other.group0().xyz(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], self[e4315], self[e4125]]) + self.group9().xy().with_zw(other[e4315], other[e4125]),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::AddAssign<Flector> for MultiVector {
    fn add_assign(&mut self, other: Flector) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3().xyz().with_w(other[e45] + self[e45]),
            // e15, e25, e35
            self.group4() + other.group0().xyz(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], self[e4315], self[e4125]]) + self.group9().xy().with_zw(other[e4315], other[e4125]),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::Add<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4() + other.group0().xyz(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::AddAssign<FlectorAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: FlectorAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4() + other.group0().xyz(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::Add<FlectorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3().xyz().with_w(other[e45] + self[e45]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], self[e4315], self[e4125]]) + self.group9().xy().with_zw(other[e4315], other[e4125]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<FlectorOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: FlectorOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3().xyz().with_w(other[e45] + self[e45]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], self[e4315], self[e4125]]) + self.group9().xy().with_zw(other[e4315], other[e4125]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::AddAssign<Horizon> for MultiVector {
    fn add_assign(&mut self, other: Horizon) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::Add<Infinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<Infinity> for MultiVector {
    fn add_assign(&mut self, other: Infinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (other.group0() + self.group6().xyz()).with_w(self[e321]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            other.group1() + self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<Line> for MultiVector {
    fn add_assign(&mut self, other: Line) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (other.group0() + self.group6().xyz()).with_w(self[e321]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            other.group1() + self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<LineAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: LineAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            other.group0() + self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<LineAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: LineAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            other.group0() + self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<LineOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: LineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (other.group0() + self.group6().xyz()).with_w(self[e321]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<LineOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: LineOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (other.group0() + self.group6().xyz()).with_w(self[e321]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd2        1        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        9        0        0
    fn add(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (other.group0().xyz() + self.group6().xyz()).with_w(self[e321]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() + other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<Motor> for MultiVector {
    fn add_assign(&mut self, other: Motor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (other.group0().xyz() + self.group6().xyz()).with_w(self[e321]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() + other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MotorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: MotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() + other.group0().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<MotorAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: MotorAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() + other.group0().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: MotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (other.group0().xyz() + self.group6().xyz()).with_w(self[e321]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<MotorOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: MotorOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (other.group0().xyz() + self.group6().xyz()).with_w(self[e321]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd2        1        0        0
    //    simd3        4        0        0
    //    simd4        4        0        0
    // Totals...
    // yes simd       11        0        0
    //  no simd       32        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            other.group0() + self.group0(),
            // e1, e2, e3, e4
            other.group1() + self.group1(),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            other.group3() + self.group3(),
            // e15, e25, e35
            other.group4() + self.group4(),
            // e23, e31, e12
            other.group5() + self.group5(),
            // e415, e425, e435, e321
            other.group6() + self.group6(),
            // e423, e431, e412
            other.group7() + self.group7(),
            // e235, e315, e125
            other.group8() + self.group8(),
            // e1234, e4235, e4315, e4125
            other.group9() + self.group9(),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::AddAssign<MultiVector> for MultiVector {
    fn add_assign(&mut self, other: MultiVector) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            other.group0() + self.group0(),
            // e1, e2, e3, e4
            other.group1() + self.group1(),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            other.group3() + self.group3(),
            // e15, e25, e35
            other.group4() + self.group4(),
            // e23, e31, e12
            other.group5() + self.group5(),
            // e415, e425, e435, e321
            other.group6() + self.group6(),
            // e423, e431, e412
            other.group7() + self.group7(),
            // e235, e315, e125
            other.group8() + self.group8(),
            // e1234, e4235, e4315, e4125
            other.group9() + self.group9(),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::Add<MysteryCircle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: MysteryCircle) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + other.group0(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<MysteryCircle> for MultiVector {
    fn add_assign(&mut self, other: MysteryCircle) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + other.group0(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MysteryCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        6        0        0
    fn add(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + other.group0(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<MysteryCircleRotor> for MultiVector {
    fn add_assign(&mut self, other: MysteryCircleRotor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + other.group0(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MysteryDipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5() + other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<MysteryDipole> for MultiVector {
    fn add_assign(&mut self, other: MysteryDipole) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5() + other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MysteryDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       11        0        0
    fn add(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5() + other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], self[e4315], self[e4125]]) + self.group9().xy().with_zw(other[e4315], other[e4125]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<MysteryDipoleInversion> for MultiVector {
    fn add_assign(&mut self, other: MysteryDipoleInversion) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5() + other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], self[e4315], self[e4125]]) + self.group9().xy().with_zw(other[e4315], other[e4125]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MysteryVersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn add(self, other: MysteryVersorEven) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]) + self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + other.group1(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<MysteryVersorEven> for MultiVector {
    fn add_assign(&mut self, other: MysteryVersorEven) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]) + self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + other.group1(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MysteryVersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        1        0        0
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       13        0        0
    fn add(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5() + other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) + self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<MysteryVersorOdd> for MultiVector {
    fn add_assign(&mut self, other: MysteryVersorOdd) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5() + other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) + self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<NullCircleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: NullCircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7() + other.group0(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<NullCircleAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: NullCircleAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7() + other.group0(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<NullDipoleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: NullDipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0() + self.group3().xyz()).with_w(self[e45]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<NullDipoleAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: NullDipoleAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            (other.group0() + self.group3().xyz()).with_w(self[e45]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<NullDipoleInversionAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + other.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<NullDipoleInversionAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: NullDipoleInversionAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + other.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<NullSphereAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<NullSphereAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: NullSphereAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<NullVersorEvenAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7() + other.group0().xyz(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<NullVersorEvenAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: NullVersorEvenAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7() + other.group0().xyz(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<Origin> for MultiVector {
    fn add_assign(&mut self, other: Origin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) + self.group9(),
            // e3215
            self[e3215] + other[e3215],
        );
    }
}
impl std::ops::AddAssign<Plane> for MultiVector {
    fn add_assign(&mut self, other: Plane) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) + self.group9(),
            // e3215
            self[e3215] + other[e3215],
        );
    }
}
impl std::ops::Add<PlaneOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], self[e4315], self[e4125]]) + self.group9().xy().with_zw(other[e4315], other[e4125]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<PlaneOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: PlaneOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], self[e4315], self[e4125]]) + self.group9().xy().with_zw(other[e4315], other[e4125]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<RoundPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + other.group0(),
            // e5
            self[e5] + other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<RoundPoint> for MultiVector {
    fn add_assign(&mut self, other: RoundPoint) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + other.group0(),
            // e5
            self[e5] + other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(self[e4] + other[e4]),
            // e5
            self[e5] + other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<RoundPointAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: RoundPointAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(self[e4] + other[e4]),
            // e5
            self[e5] + other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        1        0        0
    // no simd        2        0        0
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<Scalar> for MultiVector {
    fn add_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Sphere> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) + self.group9(),
            // e3215
            self[e3215] + other[e3215],
        );
    }
}
impl std::ops::AddAssign<Sphere> for MultiVector {
    fn add_assign(&mut self, other: Sphere) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) + self.group9(),
            // e3215
            self[e3215] + other[e3215],
        );
    }
}
impl std::ops::Add<SphereAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            self[e3215] + other[e3215],
        );
    }
}
impl std::ops::AddAssign<SphereAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: SphereAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            self[e3215] + other[e3215],
        );
    }
}
impl std::ops::Add<SphereOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9() + other.group0().wxyz(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<SphereOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: SphereOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9() + other.group0().wxyz(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd2        1        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       17        0        0
    fn add(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1() + other.group3(),
            // e5
            self[e5] + other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + other.group1(),
            // e423, e431, e412
            self.group7() + other.group0().xyz(),
            // e235, e315, e125
            self.group8() + other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<VersorEven> for MultiVector {
    fn add_assign(&mut self, other: VersorEven) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1() + other.group3(),
            // e5
            self[e5] + other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + other.group1(),
            // e423, e431, e412
            self.group7() + other.group0().xyz(),
            // e235, e315, e125
            self.group8() + other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd2        1        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       17        0        0
    fn add(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1() + Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            self[e5] + other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + other.group1().xyz().with_w(0.0),
            // e423, e431, e412
            self.group7() + other.group0().xyz(),
            // e235, e315, e125
            self.group8() + other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<VersorEvenAligningOrigin> for MultiVector {
    fn add_assign(&mut self, other: VersorEvenAligningOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1() + Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            self[e5] + other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + other.group1().xyz().with_w(0.0),
            // e423, e431, e412
            self.group7() + other.group0().xyz(),
            // e235, e315, e125
            self.group8() + other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd2        1        0        0
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       14        0        0
    fn add(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]) + self.group1(),
            // e5
            self[e5] + other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + other.group1(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() + other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<VersorEvenAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: VersorEvenAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]) + self.group1(),
            // e5
            self[e5] + other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + other.group1(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() + other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       11        0        0
    fn add(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            self[e5] + other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7() + other.group0().xyz(),
            // e235, e315, e125
            self.group8() + other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<VersorEvenAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: VersorEvenAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            self[e5] + other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7() + other.group0().xyz(),
            // e235, e315, e125
            self.group8() + other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        1        0        0
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       13        0        0
    fn add(self, other: VersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1() + Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + other.group1().xyz().with_w(0.0),
            // e423, e431, e412
            self.group7() + other.group0().xyz(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<VersorEvenOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: VersorEvenOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1() + Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + other.group1().xyz().with_w(0.0),
            // e423, e431, e412
            self.group7() + other.group0().xyz(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       15        0        0
    fn add(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + other.group2(),
            // e5
            self[e5] + other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            self.group7() + other.group0().xyz(),
            // e235, e315, e125
            self.group8() + other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<VersorEvenOrthogonalOrigin> for MultiVector {
    fn add_assign(&mut self, other: VersorEvenOrthogonalOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + other.group2(),
            // e5
            self[e5] + other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            self.group7() + other.group0().xyz(),
            // e235, e315, e125
            self.group8() + other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd2        1        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       17        0        0
    fn add(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + other.group0().xyz().with_w(other[e45]),
            // e15, e25, e35
            self.group4() + other.group2().xyz(),
            // e23, e31, e12
            self.group5() + other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) + self.group9(),
            // e3215
            self[e3215] + other[e3215],
        );
    }
}
impl std::ops::AddAssign<VersorOdd> for MultiVector {
    fn add_assign(&mut self, other: VersorOdd) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + other.group0().xyz().with_w(other[e45]),
            // e15, e25, e35
            self.group4() + other.group2().xyz(),
            // e23, e31, e12
            self.group5() + other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) + self.group9(),
            // e3215
            self[e3215] + other[e3215],
        );
    }
}
impl std::ops::Add<VersorOddAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd2        1        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       17        0        0
    fn add(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            self.group4() + other.group0().yzw(),
            // e23, e31, e12
            self.group5() + other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) + self.group9(),
            // e3215
            self[e3215] + other[e3215],
        );
    }
}
impl std::ops::AddAssign<VersorOddAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: VersorOddAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45]),
            // e15, e25, e35
            self.group4() + other.group0().yzw(),
            // e23, e31, e12
            self.group5() + other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]) + self.group9(),
            // e3215
            self[e3215] + other[e3215],
        );
    }
}
impl std::ops::Add<VersorOddOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd2        1        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       17        0        0
    fn add(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + other.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group4() + other.group2().xyz(),
            // e23, e31, e12
            self.group5() + other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            self[e3215] + other[e3215],
        );
    }
}
impl std::ops::AddAssign<VersorOddOrthogonalOrigin> for MultiVector {
    fn add_assign(&mut self, other: VersorOddOrthogonalOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + other.group0().xyz().with_w(0.0),
            // e15, e25, e35
            self.group4() + other.group2().xyz(),
            // e23, e31, e12
            self.group5() + other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            self[e3215] + other[e3215],
        );
    }
}

impl From<AntiCircleOnOrigin> for MultiVector {
    fn from(from_anti_circle_on_origin: AntiCircleOnOrigin) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_anti_circle_on_origin.group0().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            from_anti_circle_on_origin.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiCircleRotor> for MultiVector {
    fn from(from_anti_circle_rotor: AntiCircleRotor) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_circle_rotor[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_anti_circle_rotor.group0().with_w(from_anti_circle_rotor[e45]),
            // e15, e25, e35
            from_anti_circle_rotor.group2().xyz(),
            // e23, e31, e12
            from_anti_circle_rotor.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiCircleRotorAligningOrigin> for MultiVector {
    fn from(from_anti_circle_rotor_aligning_origin: AntiCircleRotorAligningOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_circle_rotor_aligning_origin[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_anti_circle_rotor_aligning_origin.group0().with_w(0.0),
            // e15, e25, e35
            from_anti_circle_rotor_aligning_origin.group2().xyz(),
            // e23, e31, e12
            from_anti_circle_rotor_aligning_origin.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiCircleRotorAligningOriginAtInfinity> for MultiVector {
    fn from(from_anti_circle_rotor_aligning_origin_at_infinity: AntiCircleRotorAligningOriginAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_circle_rotor_aligning_origin_at_infinity[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            from_anti_circle_rotor_aligning_origin_at_infinity.group1().xyz(),
            // e23, e31, e12
            from_anti_circle_rotor_aligning_origin_at_infinity.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiCircleRotorAtInfinity> for MultiVector {
    fn from(from_anti_circle_rotor_at_infinity: AntiCircleRotorAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_circle_rotor_at_infinity[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(from_anti_circle_rotor_at_infinity[e45]),
            // e15, e25, e35
            from_anti_circle_rotor_at_infinity.group1().xyz(),
            // e23, e31, e12
            from_anti_circle_rotor_at_infinity.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiCircleRotorOnOrigin> for MultiVector {
    fn from(from_anti_circle_rotor_on_origin: AntiCircleRotorOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_circle_rotor_on_origin[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_anti_circle_rotor_on_origin.group0().xyz().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            from_anti_circle_rotor_on_origin.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiDipoleInversion> for MultiVector {
    fn from(from_anti_dipole_inversion: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([
                from_anti_dipole_inversion[e1],
                from_anti_dipole_inversion[e2],
                from_anti_dipole_inversion[e3],
                from_anti_dipole_inversion[e4],
            ]),
            // e5
            from_anti_dipole_inversion[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_anti_dipole_inversion.group1(),
            // e423, e431, e412
            from_anti_dipole_inversion.group0(),
            // e235, e315, e125
            from_anti_dipole_inversion.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiDipoleInversionAtInfinity> for MultiVector {
    fn from(from_anti_dipole_inversion_at_infinity: AntiDipoleInversionAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            from_anti_dipole_inversion_at_infinity.group2().xyz().with_w(0.0),
            // e5
            from_anti_dipole_inversion_at_infinity[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_anti_dipole_inversion_at_infinity.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            from_anti_dipole_inversion_at_infinity.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiDipoleInversionOnOrigin> for MultiVector {
    fn from(from_anti_dipole_inversion_on_origin: AntiDipoleInversionOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            from_anti_dipole_inversion_on_origin.group1().yzwx(),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_dipole_inversion_on_origin[e321]),
            // e423, e431, e412
            from_anti_dipole_inversion_on_origin.group0().xyz(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiDipoleInversionOrthogonalOrigin> for MultiVector {
    fn from(from_anti_dipole_inversion_orthogonal_origin: AntiDipoleInversionOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_anti_dipole_inversion_orthogonal_origin[e4]),
            // e5
            from_anti_dipole_inversion_orthogonal_origin[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_anti_dipole_inversion_orthogonal_origin.group1().with_w(0.0),
            // e423, e431, e412
            from_anti_dipole_inversion_orthogonal_origin.group0().xyz(),
            // e235, e315, e125
            from_anti_dipole_inversion_orthogonal_origin.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiDipoleOnOrigin> for MultiVector {
    fn from(from_anti_dipole_on_origin: AntiDipoleOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_dipole_on_origin[e321]),
            // e423, e431, e412
            from_anti_dipole_on_origin.group0().xyz(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiDualNum> for MultiVector {
    fn from(from_anti_dual_num: AntiDualNum) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_dual_num[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_anti_dual_num[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}

impl From<AntiFlatOrigin> for MultiVector {
    fn from(from_anti_flat_origin: AntiFlatOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flat_origin[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiFlatPoint> for MultiVector {
    fn from(from_anti_flat_point: AntiFlatPoint) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flat_point[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            from_anti_flat_point.group0().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiFlector> for MultiVector {
    fn from(from_anti_flector: AntiFlector) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            from_anti_flector.group1().xyz().with_w(0.0),
            // e5
            from_anti_flector[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flector[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            from_anti_flector.group0().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiFlectorOnOrigin> for MultiVector {
    fn from(from_anti_flector_on_origin: AntiFlectorOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([from_anti_flector_on_origin[e1], from_anti_flector_on_origin[e2], from_anti_flector_on_origin[e3], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flector_on_origin[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiLine> for MultiVector {
    fn from(from_anti_line: AntiLine) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            from_anti_line.group1(),
            // e23, e31, e12
            from_anti_line.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiLineOnOrigin> for MultiVector {
    fn from(from_anti_line_on_origin: AntiLineOnOrigin) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            from_anti_line_on_origin.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiMotor> for MultiVector {
    fn from(from_anti_motor: AntiMotor) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_motor[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            from_anti_motor.group1().xyz(),
            // e23, e31, e12
            from_anti_motor.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            from_anti_motor[e3215],
        );
    }
}

impl From<AntiMotorOnOrigin> for MultiVector {
    fn from(from_anti_motor_on_origin: AntiMotorOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_motor_on_origin[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            from_anti_motor_on_origin.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiMysteryCircleRotor> for MultiVector {
    fn from(from_anti_mystery_circle_rotor: AntiMysteryCircleRotor) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_mystery_circle_rotor[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(from_anti_mystery_circle_rotor[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            from_anti_mystery_circle_rotor.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiMysteryDipoleInversion> for MultiVector {
    fn from(from_anti_mystery_dipole_inversion: AntiMysteryDipoleInversion) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            from_anti_mystery_dipole_inversion.group1().with_w(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_anti_mystery_dipole_inversion.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiPlane> for MultiVector {
    fn from(from_anti_plane: AntiPlane) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            from_anti_plane.group0().xyz().with_w(0.0),
            // e5
            from_anti_plane[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiPlaneOnOrigin> for MultiVector {
    fn from(from_anti_plane_on_origin: AntiPlaneOnOrigin) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            from_anti_plane_on_origin.group0().with_w(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiScalar> for MultiVector {
    fn from(from_anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_anti_scalar[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiSphereOnOrigin> for MultiVector {
    fn from(from_anti_sphere_on_origin: AntiSphereOnOrigin) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            from_anti_sphere_on_origin.group0(),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiVersorEvenOnOrigin> for MultiVector {
    fn from(from_anti_versor_even_on_origin: AntiVersorEvenOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_versor_even_on_origin[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_anti_versor_even_on_origin.group0().xyz().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            from_anti_versor_even_on_origin.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_anti_versor_even_on_origin[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}

impl From<Circle> for MultiVector {
    fn from(from_circle: Circle) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_circle.group1(),
            // e423, e431, e412
            from_circle.group0(),
            // e235, e315, e125
            from_circle.group2(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<CircleAligningOrigin> for MultiVector {
    fn from(from_circle_aligning_origin: CircleAligningOrigin) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_circle_aligning_origin.group1().with_w(0.0),
            // e423, e431, e412
            from_circle_aligning_origin.group0(),
            // e235, e315, e125
            from_circle_aligning_origin.group2(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<CircleAtInfinity> for MultiVector {
    fn from(from_circle_at_infinity: CircleAtInfinity) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_circle_at_infinity.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            from_circle_at_infinity.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<CircleAtOrigin> for MultiVector {
    fn from(from_circle_at_origin: CircleAtOrigin) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            from_circle_at_origin.group0(),
            // e235, e315, e125
            from_circle_at_origin.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<CircleOnOrigin> for MultiVector {
    fn from(from_circle_on_origin: CircleOnOrigin) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_circle_on_origin.group1().with_w(0.0),
            // e423, e431, e412
            from_circle_on_origin.group0(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<CircleOrthogonalOrigin> for MultiVector {
    fn from(from_circle_orthogonal_origin: CircleOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_circle_orthogonal_origin[e321]),
            // e423, e431, e412
            from_circle_orthogonal_origin.group0().xyz(),
            // e235, e315, e125
            from_circle_orthogonal_origin.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<CircleRotor> for MultiVector {
    fn from(from_circle_rotor: CircleRotor) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_circle_rotor[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_circle_rotor.group1(),
            // e423, e431, e412
            from_circle_rotor.group0(),
            // e235, e315, e125
            from_circle_rotor.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<CircleRotorAligningOrigin> for MultiVector {
    fn from(from_circle_rotor_aligning_origin: CircleRotorAligningOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_circle_rotor_aligning_origin[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_circle_rotor_aligning_origin.group1().with_w(0.0),
            // e423, e431, e412
            from_circle_rotor_aligning_origin.group0(),
            // e235, e315, e125
            from_circle_rotor_aligning_origin.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<CircleRotorAligningOriginAtInfinity> for MultiVector {
    fn from(from_circle_rotor_aligning_origin_at_infinity: CircleRotorAligningOriginAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_circle_rotor_aligning_origin_at_infinity[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_circle_rotor_aligning_origin_at_infinity.group0().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            from_circle_rotor_aligning_origin_at_infinity.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<CircleRotorAtInfinity> for MultiVector {
    fn from(from_circle_rotor_at_infinity: CircleRotorAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_circle_rotor_at_infinity[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_circle_rotor_at_infinity.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            from_circle_rotor_at_infinity.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<CircleRotorOnOrigin> for MultiVector {
    fn from(from_circle_rotor_on_origin: CircleRotorOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_circle_rotor_on_origin[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_circle_rotor_on_origin.group1().with_w(0.0),
            // e423, e431, e412
            from_circle_rotor_on_origin.group0().xyz(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<Dipole> for MultiVector {
    fn from(from_dipole: Dipole) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_dipole.group0().with_w(from_dipole[e45]),
            // e15, e25, e35
            from_dipole.group2(),
            // e23, e31, e12
            from_dipole.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<DipoleAligningOrigin> for MultiVector {
    fn from(from_dipole_aligning_origin: DipoleAligningOrigin) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_dipole_aligning_origin.group0(),
            // e15, e25, e35
            from_dipole_aligning_origin.group1(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<DipoleAtInfinity> for MultiVector {
    fn from(from_dipole_at_infinity: DipoleAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(from_dipole_at_infinity[e45]),
            // e15, e25, e35
            from_dipole_at_infinity.group1(),
            // e23, e31, e12
            from_dipole_at_infinity.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<DipoleAtOrigin> for MultiVector {
    fn from(from_dipole_at_origin: DipoleAtOrigin) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_dipole_at_origin.group0().with_w(0.0),
            // e15, e25, e35
            from_dipole_at_origin.group1(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<DipoleInversion> for MultiVector {
    fn from(from_dipole_inversion: DipoleInversion) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_dipole_inversion.group0().with_w(from_dipole_inversion[e45]),
            // e15, e25, e35
            from_dipole_inversion.group2().xyz(),
            // e23, e31, e12
            from_dipole_inversion.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_dipole_inversion[e1234], from_dipole_inversion[e4235], from_dipole_inversion[e4315], from_dipole_inversion[e4125]]),
            // e3215
            from_dipole_inversion[e3215],
        );
    }
}

impl From<DipoleInversionAligningOrigin> for MultiVector {
    fn from(from_dipole_inversion_aligning_origin: DipoleInversionAligningOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_dipole_inversion_aligning_origin.group0(),
            // e15, e25, e35
            from_dipole_inversion_aligning_origin.group1().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([
                from_dipole_inversion_aligning_origin[e1234],
                from_dipole_inversion_aligning_origin[e4235],
                from_dipole_inversion_aligning_origin[e4315],
                from_dipole_inversion_aligning_origin[e4125],
            ]),
            // e3215
            from_dipole_inversion_aligning_origin[e3215],
        );
    }
}

impl From<DipoleInversionAtInfinity> for MultiVector {
    fn from(from_dipole_inversion_at_infinity: DipoleInversionAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(from_dipole_inversion_at_infinity[e45]),
            // e15, e25, e35
            from_dipole_inversion_at_infinity.group1(),
            // e23, e31, e12
            from_dipole_inversion_at_infinity.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([
                0.0,
                from_dipole_inversion_at_infinity[e4235],
                from_dipole_inversion_at_infinity[e4315],
                from_dipole_inversion_at_infinity[e4125],
            ]),
            // e3215
            from_dipole_inversion_at_infinity[e3215],
        );
    }
}

impl From<DipoleInversionAtOrigin> for MultiVector {
    fn from(from_dipole_inversion_at_origin: DipoleInversionAtOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_dipole_inversion_at_origin.group0().xyz().with_w(0.0),
            // e15, e25, e35
            from_dipole_inversion_at_origin.group1().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_dipole_inversion_at_origin[e1234], 0.0, 0.0, 0.0]),
            // e3215
            from_dipole_inversion_at_origin[e3215],
        );
    }
}

impl From<DipoleInversionOnOrigin> for MultiVector {
    fn from(from_dipole_inversion_on_origin: DipoleInversionOnOrigin) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_dipole_inversion_on_origin.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            from_dipole_inversion_on_origin.group1(),
            // e3215
            0.0,
        );
    }
}

impl From<DipoleInversionOrthogonalOrigin> for MultiVector {
    fn from(from_dipole_inversion_orthogonal_origin: DipoleInversionOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_dipole_inversion_orthogonal_origin.group0().xyz().with_w(0.0),
            // e15, e25, e35
            from_dipole_inversion_orthogonal_origin.group2().xyz(),
            // e23, e31, e12
            from_dipole_inversion_orthogonal_origin.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_dipole_inversion_orthogonal_origin[e1234], 0.0, 0.0, 0.0]),
            // e3215
            from_dipole_inversion_orthogonal_origin[e3215],
        );
    }
}

impl From<DipoleOnOrigin> for MultiVector {
    fn from(from_dipole_on_origin: DipoleOnOrigin) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_dipole_on_origin.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<DipoleOrthogonalOrigin> for MultiVector {
    fn from(from_dipole_orthogonal_origin: DipoleOrthogonalOrigin) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_dipole_orthogonal_origin.group0().with_w(0.0),
            // e15, e25, e35
            from_dipole_orthogonal_origin.group2(),
            // e23, e31, e12
            from_dipole_orthogonal_origin.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<DualNum> for MultiVector {
    fn from(from_dual_num: DualNum) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_dual_num[e12345]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_dual_num[e4]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<FlatOrigin> for MultiVector {
    fn from(from_flat_origin: FlatOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(from_flat_origin[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<FlatPoint> for MultiVector {
    fn from(from_flat_point: FlatPoint) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(from_flat_point[e45]),
            // e15, e25, e35
            from_flat_point.group0().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<FlatPointAtInfinity> for MultiVector {
    fn from(from_flat_point_at_infinity: FlatPointAtInfinity) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            from_flat_point_at_infinity.group0(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<Flector> for MultiVector {
    fn from(from_flector: Flector) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(from_flector[e45]),
            // e15, e25, e35
            from_flector.group0().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, from_flector[e4235], from_flector[e4315], from_flector[e4125]]),
            // e3215
            from_flector[e3215],
        );
    }
}

impl From<FlectorAtInfinity> for MultiVector {
    fn from(from_flector_at_infinity: FlectorAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            from_flector_at_infinity.group0().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            from_flector_at_infinity[e3215],
        );
    }
}

impl From<FlectorOnOrigin> for MultiVector {
    fn from(from_flector_on_origin: FlectorOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(from_flector_on_origin[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, from_flector_on_origin[e4235], from_flector_on_origin[e4315], from_flector_on_origin[e4125]]),
            // e3215
            0.0,
        );
    }
}

impl From<Horizon> for MultiVector {
    fn from(from_horizon: Horizon) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            from_horizon[e3215],
        );
    }
}

impl From<Infinity> for MultiVector {
    fn from(from_infinity: Infinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            from_infinity[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<Line> for MultiVector {
    fn from(from_line: Line) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_line.group0().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            from_line.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<LineAtInfinity> for MultiVector {
    fn from(from_line_at_infinity: LineAtInfinity) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            from_line_at_infinity.group0(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<LineOnOrigin> for MultiVector {
    fn from(from_line_on_origin: LineOnOrigin) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_line_on_origin.group0().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<Motor> for MultiVector {
    fn from(from_motor: Motor) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_motor[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            from_motor[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_motor.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            from_motor.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<MotorAtInfinity> for MultiVector {
    fn from(from_motor_at_infinity: MotorAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            from_motor_at_infinity[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            from_motor_at_infinity.group0().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<MotorOnOrigin> for MultiVector {
    fn from(from_motor_on_origin: MotorOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_motor_on_origin[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_motor_on_origin.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<MysteryCircle> for MultiVector {
    fn from(from_mystery_circle: MysteryCircle) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_mystery_circle.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<MysteryCircleRotor> for MultiVector {
    fn from(from_mystery_circle_rotor: MysteryCircleRotor) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_mystery_circle_rotor[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_mystery_circle_rotor.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<MysteryDipole> for MultiVector {
    fn from(from_mystery_dipole: MysteryDipole) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(from_mystery_dipole[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            from_mystery_dipole.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<MysteryDipoleInversion> for MultiVector {
    fn from(from_mystery_dipole_inversion: MysteryDipoleInversion) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(from_mystery_dipole_inversion[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            from_mystery_dipole_inversion.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, from_mystery_dipole_inversion[e4235], from_mystery_dipole_inversion[e4315], from_mystery_dipole_inversion[e4125]]),
            // e3215
            0.0,
        );
    }
}

impl From<MysteryVersorEven> for MultiVector {
    fn from(from_mystery_versor_even: MysteryVersorEven) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_mystery_versor_even[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([from_mystery_versor_even[e1], from_mystery_versor_even[e2], from_mystery_versor_even[e3], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_mystery_versor_even.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<MysteryVersorOdd> for MultiVector {
    fn from(from_mystery_versor_odd: MysteryVersorOdd) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_mystery_versor_odd[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(from_mystery_versor_odd[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            from_mystery_versor_odd.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, from_mystery_versor_odd[e4235], from_mystery_versor_odd[e4315], from_mystery_versor_odd[e4125]]),
            // e3215
            0.0,
        );
    }
}

impl From<NullCircleAtOrigin> for MultiVector {
    fn from(from_null_circle_at_origin: NullCircleAtOrigin) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            from_null_circle_at_origin.group0(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<NullDipoleAtOrigin> for MultiVector {
    fn from(from_null_dipole_at_origin: NullDipoleAtOrigin) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_null_dipole_at_origin.group0().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<NullDipoleInversionAtOrigin> for MultiVector {
    fn from(from_null_dipole_inversion_at_origin: NullDipoleInversionAtOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_null_dipole_inversion_at_origin.group0().xyz().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_null_dipole_inversion_at_origin[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}

impl From<NullSphereAtOrigin> for MultiVector {
    fn from(from_null_sphere_at_origin: NullSphereAtOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_null_sphere_at_origin[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}

impl From<NullVersorEvenAtOrigin> for MultiVector {
    fn from(from_null_versor_even_at_origin: NullVersorEvenAtOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_null_versor_even_at_origin[e4]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            from_null_versor_even_at_origin.group0().xyz(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<Origin> for MultiVector {
    fn from(from_origin: Origin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_origin[e4]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<Plane> for MultiVector {
    fn from(from_plane: Plane) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, from_plane[e4235], from_plane[e4315], from_plane[e4125]]),
            // e3215
            from_plane[e3215],
        );
    }
}

impl From<PlaneOnOrigin> for MultiVector {
    fn from(from_plane_on_origin: PlaneOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, from_plane_on_origin[e4235], from_plane_on_origin[e4315], from_plane_on_origin[e4125]]),
            // e3215
            0.0,
        );
    }
}

impl From<RoundPoint> for MultiVector {
    fn from(from_round_point: RoundPoint) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            from_round_point.group0(),
            // e5
            from_round_point[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<RoundPointAtOrigin> for MultiVector {
    fn from(from_round_point_at_origin: RoundPointAtOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_round_point_at_origin[e4]),
            // e5
            from_round_point_at_origin[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<Scalar> for MultiVector {
    fn from(from_scalar: Scalar) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_scalar[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<Sphere> for MultiVector {
    fn from(from_sphere: Sphere) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_sphere[e1234], from_sphere[e4235], from_sphere[e4315], from_sphere[e4125]]),
            // e3215
            from_sphere[e3215],
        );
    }
}

impl From<SphereAtOrigin> for MultiVector {
    fn from(from_sphere_at_origin: SphereAtOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_sphere_at_origin[e1234], 0.0, 0.0, 0.0]),
            // e3215
            from_sphere_at_origin[e3215],
        );
    }
}

impl From<SphereOnOrigin> for MultiVector {
    fn from(from_sphere_on_origin: SphereOnOrigin) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            from_sphere_on_origin.group0().wxyz(),
            // e3215
            0.0,
        );
    }
}

impl From<VersorEven> for MultiVector {
    fn from(from_versor_even: VersorEven) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_versor_even[e12345]]),
            // e1, e2, e3, e4
            from_versor_even.group3(),
            // e5
            from_versor_even[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_versor_even.group1(),
            // e423, e431, e412
            from_versor_even.group0().xyz(),
            // e235, e315, e125
            from_versor_even.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<VersorEvenAligningOrigin> for MultiVector {
    fn from(from_versor_even_aligning_origin: VersorEvenAligningOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_versor_even_aligning_origin[e12345]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_versor_even_aligning_origin[e4]),
            // e5
            from_versor_even_aligning_origin[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_versor_even_aligning_origin.group1().xyz().with_w(0.0),
            // e423, e431, e412
            from_versor_even_aligning_origin.group0().xyz(),
            // e235, e315, e125
            from_versor_even_aligning_origin.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<VersorEvenAtInfinity> for MultiVector {
    fn from(from_versor_even_at_infinity: VersorEvenAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_versor_even_at_infinity[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([from_versor_even_at_infinity[e1], from_versor_even_at_infinity[e2], from_versor_even_at_infinity[e3], 0.0]),
            // e5
            from_versor_even_at_infinity[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_versor_even_at_infinity.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            from_versor_even_at_infinity.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<VersorEvenAtOrigin> for MultiVector {
    fn from(from_versor_even_at_origin: VersorEvenAtOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_versor_even_at_origin[e4]),
            // e5
            from_versor_even_at_origin[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            from_versor_even_at_origin.group0().xyz(),
            // e235, e315, e125
            from_versor_even_at_origin.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<VersorEvenOnOrigin> for MultiVector {
    fn from(from_versor_even_on_origin: VersorEvenOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_versor_even_on_origin[e12345]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_versor_even_on_origin[e4]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_versor_even_on_origin.group1().xyz().with_w(0.0),
            // e423, e431, e412
            from_versor_even_on_origin.group0().xyz(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<VersorEvenOrthogonalOrigin> for MultiVector {
    fn from(from_versor_even_orthogonal_origin: VersorEvenOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            from_versor_even_orthogonal_origin.group2(),
            // e5
            from_versor_even_orthogonal_origin[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_versor_even_orthogonal_origin[e321]),
            // e423, e431, e412
            from_versor_even_orthogonal_origin.group0().xyz(),
            // e235, e315, e125
            from_versor_even_orthogonal_origin.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<VersorOdd> for MultiVector {
    fn from(from_versor_odd: VersorOdd) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_versor_odd[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([from_versor_odd[e41], from_versor_odd[e42], from_versor_odd[e43], from_versor_odd[e45]]),
            // e15, e25, e35
            from_versor_odd.group2().xyz(),
            // e23, e31, e12
            from_versor_odd.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_versor_odd[e1234], from_versor_odd[e4235], from_versor_odd[e4315], from_versor_odd[e4125]]),
            // e3215
            from_versor_odd[e3215],
        );
    }
}

impl From<VersorOddAtInfinity> for MultiVector {
    fn from(from_versor_odd_at_infinity: VersorOddAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_versor_odd_at_infinity[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(from_versor_odd_at_infinity[e45]),
            // e15, e25, e35
            from_versor_odd_at_infinity.group0().yzw(),
            // e23, e31, e12
            from_versor_odd_at_infinity.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, from_versor_odd_at_infinity[e4235], from_versor_odd_at_infinity[e4315], from_versor_odd_at_infinity[e4125]]),
            // e3215
            from_versor_odd_at_infinity[e3215],
        );
    }
}

impl From<VersorOddOrthogonalOrigin> for MultiVector {
    fn from(from_versor_odd_orthogonal_origin: VersorOddOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_versor_odd_orthogonal_origin[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_versor_odd_orthogonal_origin.group0().xyz().with_w(0.0),
            // e15, e25, e35
            from_versor_odd_orthogonal_origin.group2().xyz(),
            // e23, e31, e12
            from_versor_odd_orthogonal_origin.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_versor_odd_orthogonal_origin[e1234], 0.0, 0.0, 0.0]),
            // e3215
            from_versor_odd_orthogonal_origin[e3215],
        );
    }
}
impl std::ops::Mul<AntiCircleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       53        0
    //    simd2        5        6        0
    //    simd3       17       24        0
    //    simd4       17       14        0
    // Totals...
    // yes simd       70       97        0
    //  no simd      160      193        0
    fn mul(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiCircleOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       82        0
    //    simd2       10       10        0
    //    simd3       40       53        0
    //    simd4       32       23        0
    // Totals...
    // yes simd      134      168        0
    //  no simd      320      353        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotor> for MultiVector {
    fn mul_assign(&mut self, other: AntiCircleRotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       54       78        0
    //    simd2        9       10        0
    //    simd3       36       46        0
    //    simd4       27       21        0
    // Totals...
    // yes simd      126      155        0
    //  no simd      288      320        0
    fn mul(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorAligningOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiCircleRotorAligningOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOriginAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       50       72        0
    //    simd2        6        7        0
    //    simd3       27       36        0
    //    simd4       13        8        0
    // Totals...
    // yes simd       96      123        0
    //  no simd      195      226        0
    fn mul(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorAligningOriginAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       49       74        0
    //    simd2        7        7        0
    //    simd3       31       42        0
    //    simd4       18       11        0
    // Totals...
    // yes simd      105      134        0
    //  no simd      228      258        0
    fn mul(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: AntiCircleRotorAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       60        0
    //    simd2        6        7        0
    //    simd3       21       33        0
    //    simd4       21       13        0
    // Totals...
    // yes simd       81      113        0
    //  no simd      192      225        0
    fn mul(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiCircleRotorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       94        0
    //    simd2        6        6        0
    //    simd3       56       75        0
    //    simd4       53       38        0
    // Totals...
    // yes simd      171      213        0
    //  no simd      448      483        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiDipoleInversion> for MultiVector {
    fn mul_assign(&mut self, other: AntiDipoleInversion) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       69       95        0
    //    simd2        4        4        0
    //    simd3       44       53        0
    //    simd4       28       23        0
    // Totals...
    // yes simd      145      175        0
    //  no simd      321      354        0
    fn mul(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiDipoleInversionAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: AntiDipoleInversionAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       59        0
    //    simd2        4        5        0
    //    simd3       24       37        0
    //    simd4       28       19        0
    // Totals...
    // yes simd       92      120        0
    //  no simd      228      256        0
    fn mul(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiDipoleInversionOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiDipoleInversionOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       82        0
    //    simd2        4        4        0
    //    simd3       40       57        0
    //    simd4       37       24        0
    // Totals...
    // yes simd      125      167        0
    //  no simd      320      357        0
    fn mul(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiDipoleInversionOrthogonalOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiDipoleInversionOrthogonalOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       50        0
    //    simd2        1        1        0
    //    simd3        9       20        0
    //    simd4       11        5        0
    // Totals...
    // yes simd       47       76        0
    //  no simd       99      132        0
    fn mul(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiDipoleOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiDipoleOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1       23        0
    //    simd2        1        2        0
    //    simd3        3        7        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       11       38        0
    //  no simd       36       72        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiDualNum> for MultiVector {
    fn mul_assign(&mut self, other: AntiDualNum) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd2        0        2        0
    //    simd3        0        7        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        0       18        0
    //  no simd        0       52        0
    fn mul(self, other: AntiFlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiFlatOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiFlatOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       44        0
    //    simd2        1        1        0
    //    simd3       18       25        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       50       73        0
    //  no simd       96      133        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiFlatPoint> for MultiVector {
    fn mul_assign(&mut self, other: AntiFlatPoint) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       57       81        0
    //    simd2        4        4        0
    //    simd3       32       40        0
    //    simd4       16       12        0
    // Totals...
    // yes simd      109      137        0
    //  no simd      225      257        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiFlector> for MultiVector {
    fn mul_assign(&mut self, other: AntiFlector) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlectorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       30        0
    //    simd2        3        4        0
    //    simd3       12       22        0
    //    simd4       10        6        0
    // Totals...
    // yes simd       39       62        0
    //  no simd       96      128        0
    fn mul(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiFlectorOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiFlectorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       70        0
    //    simd2        5        6        0
    //    simd3       23       32        0
    //    simd4        9        4        0
    // Totals...
    // yes simd       85      112        0
    //  no simd      163      194        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiLine> for MultiVector {
    fn mul_assign(&mut self, other: AntiLine) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLineOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       41        0
    //    simd2        2        3        0
    //    simd3        8       15        0
    //    simd4        4        1        0
    // Totals...
    // yes simd       34       60        0
    //  no simd       64       96        0
    fn mul(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiLineOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiLineOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       58       78        0
    //    simd2        7        8        0
    //    simd3       32       41        0
    //    simd4       15       10        0
    // Totals...
    // yes simd      112      137        0
    //  no simd      228      257        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMotor> for MultiVector {
    fn mul_assign(&mut self, other: AntiMotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       41        0
    //    simd2        3        4        0
    //    simd3       12       17        0
    //    simd4        8        7        0
    // Totals...
    // yes simd       45       69        0
    //  no simd       96      128        0
    fn mul(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMotorOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiMotorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       45        0
    //    simd2        4        4        0
    //    simd3       16       23        0
    //    simd4       13       10        0
    // Totals...
    // yes simd       53       82        0
    //  no simd      128      162        0
    fn mul(self, other: AntiMysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMysteryCircleRotor> for MultiVector {
    fn mul_assign(&mut self, other: AntiMysteryCircleRotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       64        0
    //    simd2        4        4        0
    //    simd3       24       36        0
    //    simd4       20       12        0
    // Totals...
    // yes simd       80      116        0
    //  no simd      192      228        0
    fn mul(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMysteryDipoleInversion> for MultiVector {
    fn mul_assign(&mut self, other: AntiMysteryDipoleInversion) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       43        0
    //    simd2        3        3        0
    //    simd3       13       20        0
    //    simd4        9        6        0
    // Totals...
    // yes simd       41       72        0
    //  no simd       97      133        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiPlane> for MultiVector {
    fn mul_assign(&mut self, other: AntiPlane) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlaneOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       36        0
    //    simd2        2        3        0
    //    simd3        8       14        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       28       57        0
    //  no simd       64      100        0
    fn mul(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiPlaneOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiPlaneOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd2        0        2        0
    //    simd3        0        6        0
    //    simd4        0        8        0
    // Totals...
    // yes simd        0       19        0
    //  no simd        0       57        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiScalar> for MultiVector {
    fn mul_assign(&mut self, other: AntiScalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiSphereOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       45        0
    //    simd2        3        3        0
    //    simd3       11       18        0
    //    simd4       10        7        0
    // Totals...
    // yes simd       41       73        0
    //  no simd       96      133        0
    fn mul(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiSphereOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiSphereOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorEvenOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       52        0
    //    simd2        7        8        0
    //    simd3       24       31        0
    //    simd4       27       24        0
    // Totals...
    // yes simd       88      115        0
    //  no simd      224      257        0
    fn mul(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiVersorEvenOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiVersorEvenOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       50       80        0
    //    simd2        1        1        0
    //    simd3       36       51        0
    //    simd4       32       22        0
    // Totals...
    // yes simd      119      154        0
    //  no simd      288      323        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Circle> for MultiVector {
    fn mul_assign(&mut self, other: Circle) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       65        0
    //    simd3       32       44        0
    //    simd4       29       23        0
    // Totals...
    // yes simd      105      132        0
    //  no simd      256      289        0
    fn mul(self, other: CircleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<CircleAligningOrigin> for MultiVector {
    fn mul_assign(&mut self, other: CircleAligningOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       46       67        0
    //    simd2        1        1        0
    //    simd3       27       36        0
    //    simd4       16       12        0
    // Totals...
    // yes simd       90      116        0
    //  no simd      193      225        0
    fn mul(self, other: CircleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<CircleAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: CircleAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       57        0
    //    simd3       20       29        0
    //    simd4       17       13        0
    // Totals...
    // yes simd       69       99        0
    //  no simd      160      196        0
    fn mul(self, other: CircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<CircleAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: CircleAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       37       58        0
    //    simd3       17       26        0
    //    simd4       19       14        0
    // Totals...
    // yes simd       73       98        0
    //  no simd      164      192        0
    fn mul(self, other: CircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<CircleOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: CircleOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       62        0
    //    simd2        1        1        0
    //    simd3       24       36        0
    //    simd4       21       14        0
    // Totals...
    // yes simd       80      113        0
    //  no simd      192      228        0
    fn mul(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<CircleOrthogonalOrigin> for MultiVector {
    fn mul_assign(&mut self, other: CircleOrthogonalOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       42       73        0
    //    simd2        3        3        0
    //    simd3       40       55        0
    //    simd4       38       28        0
    // Totals...
    // yes simd      123      159        0
    //  no simd      320      356        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<CircleRotor> for MultiVector {
    fn mul_assign(&mut self, other: CircleRotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       74        0
    //    simd2        2        2        0
    //    simd3       36       50        0
    //    simd4       33       24        0
    // Totals...
    // yes simd      115      150        0
    //  no simd      288      324        0
    fn mul(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<CircleRotorAligningOrigin> for MultiVector {
    fn mul_assign(&mut self, other: CircleRotorAligningOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOriginAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       64        0
    //    simd2        2        2        0
    //    simd3       27       36        0
    //    simd4       17       12        0
    // Totals...
    // yes simd       90      114        0
    //  no simd      197      224        0
    fn mul(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<CircleRotorAligningOriginAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: CircleRotorAligningOriginAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       42       63        0
    //    simd2        3        3        0
    //    simd3       31       40        0
    //    simd4       22       17        0
    // Totals...
    // yes simd       98      123        0
    //  no simd      229      257        0
    fn mul(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<CircleRotorAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: CircleRotorAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       63        0
    //    simd2        2        2        0
    //    simd3       21       36        0
    //    simd4       24       13        0
    // Totals...
    // yes simd       79      114        0
    //  no simd      195      227        0
    fn mul(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<CircleRotorOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: CircleRotorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       50       79        0
    //    simd2        9        9        0
    //    simd3       36       48        0
    //    simd4       28       20        0
    // Totals...
    // yes simd      123      156        0
    //  no simd      288      321        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Dipole> for MultiVector {
    fn mul_assign(&mut self, other: Dipole) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       49        0
    //    simd2        6        6        0
    //    simd3       24       35        0
    //    simd4       21       15        0
    // Totals...
    // yes simd       75      105        0
    //  no simd      192      226        0
    fn mul(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleAligningOrigin> for MultiVector {
    fn mul_assign(&mut self, other: DipoleAligningOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       47       71        0
    //    simd2        6        6        0
    //    simd3       27       37        0
    //    simd4       14        8        0
    // Totals...
    // yes simd       94      122        0
    //  no simd      196      226        0
    fn mul(self, other: DipoleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: DipoleAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       44        0
    //    simd2        5        6        0
    //    simd3       20       27        0
    //    simd4       16       14        0
    // Totals...
    // yes simd       67       91        0
    //  no simd      160      193        0
    fn mul(self, other: DipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: DipoleAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       74      109        0
    //    simd2       11       11        0
    //    simd3       56       75        0
    //    simd4       46       31        0
    // Totals...
    // yes simd      187      226        0
    //  no simd      448      480        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversion> for MultiVector {
    fn mul_assign(&mut self, other: DipoleInversion) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       71        0
    //    simd2        8        8        0
    //    simd3       44       59        0
    //    simd4       40       30        0
    // Totals...
    // yes simd      136      168        0
    //  no simd      352      384        0
    fn mul(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionAligningOrigin> for MultiVector {
    fn mul_assign(&mut self, other: DipoleInversionAligningOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       70      101        0
    //    simd2        7        7        0
    //    simd3       44       58        0
    //    simd4       26       16        0
    // Totals...
    // yes simd      147      182        0
    //  no simd      320      353        0
    fn mul(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: DipoleInversionAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       43        0
    //    simd2        7        8        0
    //    simd3       28       35        0
    //    simd4       25       23        0
    // Totals...
    // yes simd       86      109        0
    //  no simd      224      256        0
    fn mul(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: DipoleInversionAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       39       64        0
    //    simd2        4        4        0
    //    simd3       24       35        0
    //    simd4       27       20        0
    // Totals...
    // yes simd       94      123        0
    //  no simd      227      257        0
    fn mul(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: DipoleInversionOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       79        0
    //    simd2       10       11        0
    //    simd3       40       53        0
    //    simd4       32       23        0
    // Totals...
    // yes simd      134      166        0
    //  no simd      320      352        0
    fn mul(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionOrthogonalOrigin> for MultiVector {
    fn mul_assign(&mut self, other: DipoleInversionOrthogonalOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       32        0
    //    simd2        3        3        0
    //    simd3        9       18        0
    //    simd4       15       12        0
    // Totals...
    // yes simd       37       65        0
    //  no simd      103      140        0
    fn mul(self, other: DipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: DipoleOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       75        0
    //    simd2        8        9        0
    //    simd3       32       41        0
    //    simd4       23       18        0
    // Totals...
    // yes simd      115      143        0
    //  no simd      256      288        0
    fn mul(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleOrthogonalOrigin> for MultiVector {
    fn mul_assign(&mut self, other: DipoleOrthogonalOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       35        0
    //    simd3        3        8        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       17       46        0
    //  no simd       32       71        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DualNum> for MultiVector {
    fn mul_assign(&mut self, other: DualNum) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd2        0        2        0
    //    simd3        0        6        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        0       18        0
    //  no simd        0       50        0
    fn mul(self, other: FlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlatOrigin> for MultiVector {
    fn mul_assign(&mut self, other: FlatOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       44        0
    //    simd2        3        3        0
    //    simd3       18       25        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       51       73        0
    //  no simd       96      129        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlatPoint> for MultiVector {
    fn mul_assign(&mut self, other: FlatPoint) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       33        0
    //    simd2        2        3        0
    //    simd3       12       19        0
    //    simd4        3        0        0
    // Totals...
    // yes simd       39       55        0
    //  no simd       74       96        0
    fn mul(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlatPointAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: FlatPointAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       49       66        0
    //    simd2        4        4        0
    //    simd3       32       42        0
    //    simd4       19       14        0
    // Totals...
    // yes simd      104      126        0
    //  no simd      229      256        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Flector> for MultiVector {
    fn mul_assign(&mut self, other: Flector) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       41        0
    //    simd2        3        4        0
    //    simd3       17       25        0
    //    simd4        5        1        0
    // Totals...
    // yes simd       55       71        0
    //  no simd      107      128        0
    fn mul(self, other: FlectorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlectorAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: FlectorAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       41        0
    //    simd2        2        2        0
    //    simd3       12       21        0
    //    simd4        9        5        0
    // Totals...
    // yes simd       43       69        0
    //  no simd       96      128        0
    fn mul(self, other: FlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlectorOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: FlectorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        8        0
    //    simd2        0        1        0
    //    simd3        2        6        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        4       18        0
    //  no simd        8       40        0
    fn mul(self, other: Horizon) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Horizon> for MultiVector {
    fn mul_assign(&mut self, other: Horizon) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Infinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        8        0
    //    simd2        0        2        0
    //    simd3        2        7        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4       19        0
    //  no simd        8       41        0
    fn mul(self, other: Infinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Infinity> for MultiVector {
    fn mul_assign(&mut self, other: Infinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       62        0
    //    simd3       23       29        0
    //    simd4       13       11        0
    // Totals...
    // yes simd       77      102        0
    //  no simd      162      193        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Line> for MultiVector {
    fn mul_assign(&mut self, other: Line) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<LineAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       38        0
    //    simd3       12       18        0
    //    simd4        3        1        0
    // Totals...
    // yes simd       41       57        0
    //  no simd       74       96        0
    fn mul(self, other: LineAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<LineAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: LineAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<LineOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       37        0
    //    simd3        8       13        0
    //    simd4        8        7        0
    // Totals...
    // yes simd       24       57        0
    //  no simd       64      104        0
    fn mul(self, other: LineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<LineOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: LineOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       53       77        0
    //    simd2        4        5        0
    //    simd3       32       43        0
    //    simd4       18       10        0
    // Totals...
    // yes simd      107      135        0
    //  no simd      229      256        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Motor> for MultiVector {
    fn mul_assign(&mut self, other: Motor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       44        0
    //    simd2        2        2        0
    //    simd3       17       24        0
    //    simd4        5        2        0
    // Totals...
    // yes simd       56       72        0
    //  no simd      107      128        0
    fn mul(self, other: MotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MotorAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: MotorAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       18        0
    //    simd2        2        2        0
    //    simd3       12       22        0
    //    simd4       12       10        0
    // Totals...
    // yes simd       34       52        0
    //  no simd       96      128        0
    fn mul(self, other: MotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MotorOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: MotorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      156      207        0
    //    simd2       16       17        0
    //    simd3      124      157        0
    //    simd4      108       78        0
    // Totals...
    // yes simd      404      459        0
    //  no simd      992     1024        0
    fn mul(self, other: MultiVector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MultiVector> for MultiVector {
    fn mul_assign(&mut self, other: MultiVector) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       42        0
    //    simd2        1        1        0
    //    simd3       12       16        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       41       69        0
    //  no simd       96      132        0
    fn mul(self, other: MysteryCircle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryCircle> for MultiVector {
    fn mul_assign(&mut self, other: MysteryCircle) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       33        0
    //    simd2        1        1        0
    //    simd3       16       23        0
    //    simd4       16       15        0
    // Totals...
    // yes simd       47       72        0
    //  no simd      128      164        0
    fn mul(self, other: MysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryCircleRotor> for MultiVector {
    fn mul_assign(&mut self, other: MysteryCircleRotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       40        0
    //    simd2        3        3        0
    //    simd3       12       16        0
    //    simd4        9        9        0
    // Totals...
    // yes simd       42       68        0
    //  no simd       96      130        0
    fn mul(self, other: MysteryDipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryDipole> for MultiVector {
    fn mul_assign(&mut self, other: MysteryDipole) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       46       69        0
    //    simd2        3        3        0
    //    simd3       24       35        0
    //    simd4       17       11        0
    // Totals...
    // yes simd       90      118        0
    //  no simd      192      224        0
    fn mul(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryDipoleInversion> for MultiVector {
    fn mul_assign(&mut self, other: MysteryDipoleInversion) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       43        0
    //    simd2        4        4        0
    //    simd3       28       35        0
    //    simd4       26       25        0
    // Totals...
    // yes simd       86      107        0
    //  no simd      224      256        0
    fn mul(self, other: MysteryVersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryVersorEven> for MultiVector {
    fn mul_assign(&mut self, other: MysteryVersorEven) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       67        0
    //    simd2        4        4        0
    //    simd3       28       35        0
    //    simd4       21       19        0
    // Totals...
    // yes simd      101      125        0
    //  no simd      224      256        0
    fn mul(self, other: MysteryVersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryVersorOdd> for MultiVector {
    fn mul_assign(&mut self, other: MysteryVersorOdd) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<NullCircleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       47        0
    //    simd3        7       16        0
    //    simd4        8        1        0
    // Totals...
    // yes simd       37       64        0
    //  no simd       75       99        0
    fn mul(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<NullCircleAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: NullCircleAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       31        0
    //    simd2        2        3        0
    //    simd3        7       14        0
    //    simd4       10        5        0
    // Totals...
    // yes simd       30       53        0
    //  no simd       76       99        0
    fn mul(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<NullDipoleAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: NullDipoleAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleInversionAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       23        0
    //    simd2        3        4        0
    //    simd3       10       15        0
    //    simd4       15       13        0
    // Totals...
    // yes simd       43       55        0
    //  no simd      111      128        0
    fn mul(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<NullDipoleInversionAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: NullDipoleInversionAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<NullSphereAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        8        0
    //    simd2        0        1        0
    //    simd3        2        7        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4       18        0
    //  no simd        8       39        0
    fn mul(self, other: NullSphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<NullSphereAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: NullSphereAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<NullVersorEvenAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       38        0
    //    simd2        2        2        0
    //    simd3       10       15        0
    //    simd4       13       11        0
    // Totals...
    // yes simd       46       66        0
    //  no simd      107      131        0
    fn mul(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<NullVersorEvenAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: NullVersorEvenAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd2        0        2        0
    //    simd3        2        8        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        4       22        0
    //  no simd        8       49        0
    fn mul(self, other: Origin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Origin> for MultiVector {
    fn mul_assign(&mut self, other: Origin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       49        0
    //    simd2        1        1        0
    //    simd3       13       17        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       48       74        0
    //  no simd       96      130        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Plane> for MultiVector {
    fn mul_assign(&mut self, other: Plane) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<PlaneOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       44        0
    //    simd3        8       15        0
    //    simd4        5        2        0
    // Totals...
    // yes simd       33       61        0
    //  no simd       64       97        0
    fn mul(self, other: PlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<PlaneOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: PlaneOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       48        0
    //    simd2        3        3        0
    //    simd3       16       22        0
    //    simd4       13       11        0
    // Totals...
    // yes simd       54       84        0
    //  no simd      128      164        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<RoundPoint> for MultiVector {
    fn mul_assign(&mut self, other: RoundPoint) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       25        0
    //    simd3        4        8        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       12       38        0
    //  no simd       32       69        0
    fn mul(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<RoundPointAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: RoundPointAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       11        0
    //  no simd        0       32        0
    fn mul(self, other: Scalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for MultiVector {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       44        0
    //    simd2        2        2        0
    //    simd3       16       25        0
    //    simd4       14       10        0
    // Totals...
    // yes simd       52       81        0
    //  no simd      128      163        0
    fn mul(self, other: Sphere) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Sphere> for MultiVector {
    fn mul_assign(&mut self, other: Sphere) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       16        0
    //    simd2        1        2        0
    //    simd3        4        8        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       11       32        0
    //  no simd       32       68        0
    fn mul(self, other: SphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<SphereAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: SphereAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       39        0
    //    simd2        1        1        0
    //    simd3       11       17        0
    //    simd4       12       10        0
    // Totals...
    // yes simd       40       67        0
    //  no simd       99      132        0
    fn mul(self, other: SphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<SphereOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: SphereOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       80        0
    //    simd2        8        8        0
    //    simd3       60       76        0
    //    simd4       57       47        0
    // Totals...
    // yes simd      181      211        0
    //  no simd      480      512        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorEven> for MultiVector {
    fn mul_assign(&mut self, other: VersorEven) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       70        0
    //    simd2        6        6        0
    //    simd3       44       58        0
    //    simd4       41       32        0
    // Totals...
    // yes simd      135      166        0
    //  no simd      352      384        0
    fn mul(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorEvenAligningOrigin> for MultiVector {
    fn mul_assign(&mut self, other: VersorEvenAligningOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       69       95        0
    //    simd2        6        6        0
    //    simd3       48       58        0
    //    simd4       32       26        0
    // Totals...
    // yes simd      155      185        0
    //  no simd      353      385        0
    fn mul(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorEvenAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: VersorEvenAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       65        0
    //    simd2        4        4        0
    //    simd3       28       41        0
    //    simd4       23       15        0
    // Totals...
    // yes simd       95      125        0
    //  no simd      224      256        0
    fn mul(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorEvenAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: VersorEvenAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       55        0
    //    simd2        4        5        0
    //    simd3       24       34        0
    //    simd4       29       23        0
    // Totals...
    // yes simd       88      117        0
    //  no simd      227      259        0
    fn mul(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorEvenOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: VersorEvenOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       77        0
    //    simd2        6        6        0
    //    simd3       44       61        0
    //    simd4       39       28        0
    // Totals...
    // yes simd      141      172        0
    //  no simd      352      384        0
    fn mul(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorEvenOrthogonalOrigin> for MultiVector {
    fn mul_assign(&mut self, other: VersorEvenOrthogonalOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       76      108        0
    //    simd2       12       13        0
    //    simd3       60       74        0
    //    simd4       50       39        0
    // Totals...
    // yes simd      198      234        0
    //  no simd      480      512        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorOdd> for MultiVector {
    fn mul_assign(&mut self, other: VersorOdd) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       72       99        0
    //    simd2        8        8        0
    //    simd3       48       58        0
    //    simd4       30       24        0
    // Totals...
    // yes simd      158      189        0
    //  no simd      352      385        0
    fn mul(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorOddAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: VersorOddAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       54       80        0
    //    simd2       11       13        0
    //    simd3       44       54        0
    //    simd4       36       29        0
    // Totals...
    // yes simd      145      176        0
    //  no simd      352      384        0
    fn mul(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorOddOrthogonalOrigin> for MultiVector {
    fn mul_assign(&mut self, other: VersorOddOrthogonalOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Neg for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       11        0
    //  no simd        0       32        0
    fn neg(self) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0() * Simd32x2::from(-1.0),
            // e1, e2, e3, e4
            self.group1() * Simd32x4::from(-1.0),
            // e5
            self[e5] * -1.0,
            // e41, e42, e43, e45
            self.group3() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group4() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group5() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group6() * Simd32x4::from(-1.0),
            // e423, e431, e412
            self.group7() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group8() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            self.group9() * Simd32x4::from(-1.0),
            // e3215
            self[e3215] * -1.0,
        );
    }
}
impl std::ops::Not for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       22        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiCircleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        7        3        0
    fn sub(self, other: AntiCircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5() - other.group1(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiCircleOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiCircleOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5() - other.group1(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        1        0        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        1        0
    //  no simd       12        1        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() - other.group0().with_w(other[e45]),
            // e15, e25, e35
            self.group4() - other.group2().xyz(),
            // e23, e31, e12
            self.group5() - other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiCircleRotor> for MultiVector {
    fn sub_assign(&mut self, other: AntiCircleRotor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() - other.group0().with_w(other[e45]),
            // e15, e25, e35
            self.group4() - other.group2().xyz(),
            // e23, e31, e12
            self.group5() - other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd2        1        0        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        4        0
    //  no simd       12        4        0
    fn sub(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group3(),
            // e15, e25, e35
            self.group4() - other.group2().xyz(),
            // e23, e31, e12
            self.group5() - other.group1(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiCircleRotorAligningOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiCircleRotorAligningOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group3(),
            // e15, e25, e35
            self.group4() - other.group2().xyz(),
            // e23, e31, e12
            self.group5() - other.group1(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOriginAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        1        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        8        1        0
    fn sub(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4() - other.group1().xyz(),
            // e23, e31, e12
            self.group5() - other.group0(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiCircleRotorAligningOriginAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4() - other.group1().xyz(),
            // e23, e31, e12
            self.group5() - other.group0(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        1        0        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        2        0
    //  no simd       12        2        0
    fn sub(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35
            self.group4() - other.group1().xyz(),
            // e23, e31, e12
            self.group5() - other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiCircleRotorAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: AntiCircleRotorAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35
            self.group4() - other.group1().xyz(),
            // e23, e31, e12
            self.group5() - other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiCircleRotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd2        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        9        4        0
    fn sub(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5() - other.group1(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiCircleRotorOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiCircleRotorOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5() - other.group1(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       15        0        0
    fn sub(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group3().xyz().with_w(other[e4]),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group1(),
            // e423, e431, e412
            self.group7() - other.group0(),
            // e235, e315, e125
            self.group8() - other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiDipoleInversion> for MultiVector {
    fn sub_assign(&mut self, other: AntiDipoleInversion) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group3().xyz().with_w(other[e4]),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group1(),
            // e423, e431, e412
            self.group7() - other.group0(),
            // e235, e315, e125
            self.group8() - other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        4        3        0
    //  no simd       12        3        0
    fn sub(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group1(),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group0(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() - other.group1(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiDipoleInversionAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: AntiDipoleInversionAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group1(),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group0(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() - other.group1(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        1        0
    //  no simd       11        1        0
    fn sub(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group1().yzwx(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412
            self.group7() - other.group0().xyz(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiDipoleInversionOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiDipoleInversionOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group1().yzwx(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412
            self.group7() - other.group0().xyz(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        4        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        4        0
    //  no simd       15        4        0
    fn sub(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group6(),
            // e423, e431, e412
            self.group7() - other.group0().xyz(),
            // e235, e315, e125
            self.group8() - other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiDipoleInversionOrthogonalOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiDipoleInversionOrthogonalOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group6(),
            // e423, e431, e412
            self.group7() - other.group0().xyz(),
            // e235, e315, e125
            self.group8() - other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDipoleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        7        1        0
    fn sub(self, other: AntiDipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412
            self.group7() - other.group0().xyz(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiDipoleOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiDipoleOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412
            self.group7() - other.group0().xyz(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        6        2        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] * -1.0, 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiDualNum> for MultiVector {
    fn sub_assign(&mut self, other: AntiDualNum) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] * -1.0, 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiFlatOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiFlatOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiFlatOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiFlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        7        1        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() - other.group0().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiFlatPoint> for MultiVector {
    fn sub_assign(&mut self, other: AntiFlatPoint) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() - other.group0().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiFlector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        4        0
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        4        4        0
    //  no simd       12        4        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group1(),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() - other.group0().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiFlector> for MultiVector {
    fn sub_assign(&mut self, other: AntiFlector) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group1(),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() - other.group0().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiFlectorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        4        0
    fn sub(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiFlectorOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiFlectorOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiLine> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn sub(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4() - other.group1(),
            // e23, e31, e12
            self.group5() - other.group0(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiLine> for MultiVector {
    fn sub_assign(&mut self, other: AntiLine) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4() - other.group1(),
            // e23, e31, e12
            self.group5() - other.group0(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiLineOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn sub(self, other: AntiLineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5() - other.group0(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiLineOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiLineOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5() - other.group0(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiMotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        1        0
    //    simd2        1        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        9        1        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4() - other.group1().xyz(),
            // e23, e31, e12
            self.group5() - other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiMotor> for MultiVector {
    fn sub_assign(&mut self, other: AntiMotor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4() - other.group1().xyz(),
            // e23, e31, e12
            self.group5() - other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<AntiMotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        5        1        0
    fn sub(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5() - other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiMotorOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiMotorOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5() - other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiMysteryCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        9        2        0
    fn sub(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5() - other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiMysteryCircleRotor> for MultiVector {
    fn sub_assign(&mut self, other: AntiMysteryCircleRotor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5() - other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiMysteryDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        8        3        0
    fn sub(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group0(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiMysteryDipoleInversion> for MultiVector {
    fn sub_assign(&mut self, other: AntiMysteryDipoleInversion) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group0(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiPlane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        5        3        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group1(),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiPlane> for MultiVector {
    fn sub_assign(&mut self, other: AntiPlane) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group1(),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiPlaneOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        4        3        0
    fn sub(self, other: AntiPlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiPlaneOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiPlaneOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        2        1        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiScalar> for MultiVector {
    fn sub_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiSphereOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: AntiSphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group0(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiSphereOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiSphereOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group0(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiVersorEvenOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd2        1        0        0
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        4        5        0
    //  no simd       13        5        0
    fn sub(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5() - other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] * -1.0, 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiVersorEvenOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiVersorEvenOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5() - other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] * -1.0, 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Circle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn sub(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group1(),
            // e423, e431, e412
            self.group7() - other.group0(),
            // e235, e315, e125
            self.group8() - other.group2(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<Circle> for MultiVector {
    fn sub_assign(&mut self, other: Circle) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group1(),
            // e423, e431, e412
            self.group7() - other.group0(),
            // e235, e315, e125
            self.group8() - other.group2(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        3        0
    //  no simd       10        3        0
    fn sub(self, other: CircleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group6(),
            // e423, e431, e412
            self.group7() - other.group0(),
            // e235, e315, e125
            self.group8() - other.group2(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<CircleAligningOrigin> for MultiVector {
    fn sub_assign(&mut self, other: CircleAligningOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group6(),
            // e423, e431, e412
            self.group7() - other.group0(),
            // e235, e315, e125
            self.group8() - other.group2(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn sub(self, other: CircleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group0(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() - other.group1(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<CircleAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: CircleAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group0(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() - other.group1(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn sub(self, other: CircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7() - other.group0(),
            // e235, e315, e125
            self.group8() - other.group1(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<CircleAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: CircleAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7() - other.group0(),
            // e235, e315, e125
            self.group8() - other.group1(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        7        3        0
    fn sub(self, other: CircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group6(),
            // e423, e431, e412
            self.group7() - other.group0(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<CircleOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: CircleOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group6(),
            // e423, e431, e412
            self.group7() - other.group0(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        1        0
    //  no simd       10        1        0
    fn sub(self, other: CircleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412
            self.group7() - other.group0().xyz(),
            // e235, e315, e125
            self.group8() - other.group1(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<CircleOrthogonalOrigin> for MultiVector {
    fn sub_assign(&mut self, other: CircleOrthogonalOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412
            self.group7() - other.group0().xyz(),
            // e235, e315, e125
            self.group8() - other.group1(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        1        0        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        1        0
    //  no simd       12        1        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group1(),
            // e423, e431, e412
            self.group7() - other.group0(),
            // e235, e315, e125
            self.group8() - other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<CircleRotor> for MultiVector {
    fn sub_assign(&mut self, other: CircleRotor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group1(),
            // e423, e431, e412
            self.group7() - other.group0(),
            // e235, e315, e125
            self.group8() - other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd2        1        0        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        4        0
    //  no simd       12        4        0
    fn sub(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group6(),
            // e423, e431, e412
            self.group7() - other.group0(),
            // e235, e315, e125
            self.group8() - other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<CircleRotorAligningOrigin> for MultiVector {
    fn sub_assign(&mut self, other: CircleRotorAligningOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group6(),
            // e423, e431, e412
            self.group7() - other.group0(),
            // e235, e315, e125
            self.group8() - other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOriginAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd2        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        9        4        0
    fn sub(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() - other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<CircleRotorAligningOriginAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: CircleRotorAligningOriginAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() - other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        9        1        0
    fn sub(self, other: CircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group0(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() - other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<CircleRotorAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: CircleRotorAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group0(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() - other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd2        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        9        4        0
    fn sub(self, other: CircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group6(),
            // e423, e431, e412
            self.group7() - other.group0().xyz(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<CircleRotorOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: CircleRotorOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group6(),
            // e423, e431, e412
            self.group7() - other.group0().xyz(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Dipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn sub(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() - other.group0().with_w(other[e45]),
            // e15, e25, e35
            self.group4() - other.group2(),
            // e23, e31, e12
            self.group5() - other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<Dipole> for MultiVector {
    fn sub_assign(&mut self, other: Dipole) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() - other.group0().with_w(other[e45]),
            // e15, e25, e35
            self.group4() - other.group2(),
            // e23, e31, e12
            self.group5() - other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<DipoleAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn sub(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() - other.group0(),
            // e15, e25, e35
            self.group4() - other.group1(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<DipoleAligningOrigin> for MultiVector {
    fn sub_assign(&mut self, other: DipoleAligningOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() - other.group0(),
            // e15, e25, e35
            self.group4() - other.group1(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<DipoleAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        1        0
    //  no simd       10        1        0
    fn sub(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35
            self.group4() - other.group1(),
            // e23, e31, e12
            self.group5() - other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<DipoleAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: DipoleAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35
            self.group4() - other.group1(),
            // e23, e31, e12
            self.group5() - other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<DipoleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        7        3        0
    fn sub(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group3(),
            // e15, e25, e35
            self.group4() - other.group1(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<DipoleAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: DipoleAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group3(),
            // e15, e25, e35
            self.group4() - other.group1(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<DipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        4        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        4        0
    //  no simd       15        4        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() - other.group0().with_w(other[e45]),
            // e15, e25, e35
            self.group4() - other.group2().xyz(),
            // e23, e31, e12
            self.group5() - other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] * -1.0, other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0]) + self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<DipoleInversion> for MultiVector {
    fn sub_assign(&mut self, other: DipoleInversion) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() - other.group0().with_w(other[e45]),
            // e15, e25, e35
            self.group4() - other.group2().xyz(),
            // e23, e31, e12
            self.group5() - other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] * -1.0, other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0]) + self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<DipoleInversionAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        4        0
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        4        4        0
    //  no simd       12        4        0
    fn sub(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() - other.group0(),
            // e15, e25, e35
            self.group4() - other.group1().xyz(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] * -1.0, other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0]) + self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<DipoleInversionAligningOrigin> for MultiVector {
    fn sub_assign(&mut self, other: DipoleInversionAligningOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() - other.group0(),
            // e15, e25, e35
            self.group4() - other.group1().xyz(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] * -1.0, other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0]) + self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<DipoleInversionAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        4        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        4        0
    //  no simd       15        4        0
    fn sub(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35
            self.group4() - other.group1(),
            // e23, e31, e12
            self.group5() - other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0]) + self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<DipoleInversionAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: DipoleInversionAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35
            self.group4() - other.group1(),
            // e23, e31, e12
            self.group5() - other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0]) + self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<DipoleInversionAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        4        0
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        4        4        0
    //  no simd       12        4        0
    fn sub(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group3(),
            // e15, e25, e35
            self.group4() - other.group1().xyz(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] * -1.0, 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<DipoleInversionAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: DipoleInversionAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group3(),
            // e15, e25, e35
            self.group4() - other.group1().xyz(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] * -1.0, 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<DipoleInversionOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn sub(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() - other.group0(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9() - other.group1(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<DipoleInversionOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: DipoleInversionOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() - other.group0(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9() - other.group1(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<DipoleInversionOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        4        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        4        0
    //  no simd       15        4        0
    fn sub(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group3(),
            // e15, e25, e35
            self.group4() - other.group2().xyz(),
            // e23, e31, e12
            self.group5() - other.group1(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] * -1.0, 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<DipoleInversionOrthogonalOrigin> for MultiVector {
    fn sub_assign(&mut self, other: DipoleInversionOrthogonalOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group3(),
            // e15, e25, e35
            self.group4() - other.group2().xyz(),
            // e23, e31, e12
            self.group5() - other.group1(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] * -1.0, 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<DipoleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() - other.group0(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<DipoleOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: DipoleOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() - other.group0(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<DipoleOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        3        0
    //  no simd       10        3        0
    fn sub(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group3(),
            // e15, e25, e35
            self.group4() - other.group2(),
            // e23, e31, e12
            self.group5() - other.group1(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<DipoleOrthogonalOrigin> for MultiVector {
    fn sub_assign(&mut self, other: DipoleOrthogonalOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group3(),
            // e15, e25, e35
            self.group4() - other.group2(),
            // e23, e31, e12
            self.group5() - other.group1(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        6        2        0
    fn sub(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<DualNum> for MultiVector {
    fn sub_assign(&mut self, other: DualNum) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<FlatOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<FlatOrigin> for MultiVector {
    fn sub_assign(&mut self, other: FlatOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<FlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        7        1        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35
            self.group4() - other.group0().xyz(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<FlatPoint> for MultiVector {
    fn sub_assign(&mut self, other: FlatPoint) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35
            self.group4() - other.group0().xyz(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn sub(self, other: FlatPointAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4() - other.group0(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<FlatPointAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: FlatPointAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4() - other.group0(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        4        0
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        4        4        0
    //  no simd       12        4        0
    fn sub(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35
            self.group4() - other.group0().xyz(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0]) + self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<Flector> for MultiVector {
    fn sub_assign(&mut self, other: Flector) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35
            self.group4() - other.group0().xyz(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0]) + self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn sub(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4() - other.group0().xyz(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<FlectorAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: FlectorAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4() - other.group0().xyz(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<FlectorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        4        0
    fn sub(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0]) + self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<FlectorOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: FlectorOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0]) + self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<Horizon> for MultiVector {
    fn sub_assign(&mut self, other: Horizon) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<Infinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<Infinity> for MultiVector {
    fn sub_assign(&mut self, other: Infinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        7        3        0
    fn sub(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() - other.group1(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<Line> for MultiVector {
    fn sub_assign(&mut self, other: Line) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() - other.group1(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<LineAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn sub(self, other: LineAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() - other.group0(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<LineAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: LineAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() - other.group0(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<LineOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        4        3        0
    fn sub(self, other: LineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<LineOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: LineOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        4        0
    //    simd2        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        4        0
    //  no simd       10        4        0
    fn sub(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() - other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<Motor> for MultiVector {
    fn sub_assign(&mut self, other: Motor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() - other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MotorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn sub(self, other: MotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() - other.group0().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<MotorAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: MotorAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() - other.group0().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd2        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        6        4        0
    fn sub(self, other: MotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<MotorOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: MotorOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd2        1        0        0
    //    simd3        4        0        0
    //    simd4        4        0        0
    // Totals...
    // yes simd       11        0        0
    //  no simd       32        0        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0() - other.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group1(),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3() - other.group3(),
            // e15, e25, e35
            self.group4() - other.group4(),
            // e23, e31, e12
            self.group5() - other.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group6(),
            // e423, e431, e412
            self.group7() - other.group7(),
            // e235, e315, e125
            self.group8() - other.group8(),
            // e1234, e4235, e4315, e4125
            self.group9() - other.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<MultiVector> for MultiVector {
    fn sub_assign(&mut self, other: MultiVector) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0() - other.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group1(),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3() - other.group3(),
            // e15, e25, e35
            self.group4() - other.group4(),
            // e23, e31, e12
            self.group5() - other.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group6(),
            // e423, e431, e412
            self.group7() - other.group7(),
            // e235, e315, e125
            self.group8() - other.group8(),
            // e1234, e4235, e4315, e4125
            self.group9() - other.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<MysteryCircle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: MysteryCircle) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group0(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<MysteryCircle> for MultiVector {
    fn sub_assign(&mut self, other: MysteryCircle) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group0(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MysteryCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        6        1        0
    fn sub(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group0(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<MysteryCircleRotor> for MultiVector {
    fn sub_assign(&mut self, other: MysteryCircleRotor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group0(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MysteryDipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        7        1        0
    fn sub(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5() - other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<MysteryDipole> for MultiVector {
    fn sub_assign(&mut self, other: MysteryDipole) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5() - other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MysteryDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        4        0
    //  no simd       11        4        0
    fn sub(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5() - other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0]) + self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<MysteryDipoleInversion> for MultiVector {
    fn sub_assign(&mut self, other: MysteryDipoleInversion) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5() - other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0]) + self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MysteryVersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd2        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        4        0
    //  no simd       10        4        0
    fn sub(self, other: MysteryVersorEven) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group1(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<MysteryVersorEven> for MultiVector {
    fn sub_assign(&mut self, other: MysteryVersorEven) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group1(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MysteryVersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd2        1        0        0
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        4        5        0
    //  no simd       13        5        0
    fn sub(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5() - other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0]) + self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<MysteryVersorOdd> for MultiVector {
    fn sub_assign(&mut self, other: MysteryVersorOdd) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5() - other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0]) + self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<NullCircleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn sub(self, other: NullCircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7() - other.group0(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<NullCircleAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: NullCircleAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7() - other.group0(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<NullDipoleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        4        3        0
    fn sub(self, other: NullDipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<NullDipoleAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: NullDipoleAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<NullDipoleInversionAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        8        4        0
    fn sub(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] * -1.0, 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<NullDipoleInversionAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: NullDipoleInversionAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] * -1.0, 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<NullSphereAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] * -1.0, 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<NullSphereAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: NullSphereAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] * -1.0, 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<NullVersorEvenAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        7        1        0
    fn sub(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7() - other.group0().xyz(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<NullVersorEvenAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: NullVersorEvenAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7() - other.group0().xyz(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<Origin> for MultiVector {
    fn sub_assign(&mut self, other: Origin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        5        3        0
    fn sub(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0]) + self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<Plane> for MultiVector {
    fn sub_assign(&mut self, other: Plane) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0]) + self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<PlaneOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        4        3        0
    fn sub(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0]) + self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<PlaneOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: PlaneOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0]) + self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<RoundPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group0(),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<RoundPoint> for MultiVector {
    fn sub_assign(&mut self, other: RoundPoint) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group0(),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        5        1        0
    fn sub(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<RoundPointAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: RoundPointAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        2        1        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<Scalar> for MultiVector {
    fn sub_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Sphere> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        4        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        5        4        0
    fn sub(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] * -1.0, other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0]) + self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<Sphere> for MultiVector {
    fn sub_assign(&mut self, other: Sphere) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] * -1.0, other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0]) + self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<SphereAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        5        1        0
    fn sub(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] * -1.0, 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<SphereAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: SphereAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] * -1.0, 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<SphereOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9() - other.group0().wxyz(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<SphereOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: SphereOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9() - other.group0().wxyz(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        1        0
    //    simd2        1        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        1        0
    //  no simd       17        1        0
    fn sub(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group3(),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group1(),
            // e423, e431, e412
            self.group7() - other.group0().xyz(),
            // e235, e315, e125
            self.group8() - other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<VersorEven> for MultiVector {
    fn sub_assign(&mut self, other: VersorEven) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group3(),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group1(),
            // e423, e431, e412
            self.group7() - other.group0().xyz(),
            // e235, e315, e125
            self.group8() - other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd2        1        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        5        0
    //  no simd       17        5        0
    fn sub(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group6(),
            // e423, e431, e412
            self.group7() - other.group0().xyz(),
            // e235, e315, e125
            self.group8() - other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<VersorEvenAligningOrigin> for MultiVector {
    fn sub_assign(&mut self, other: VersorEvenAligningOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group6(),
            // e423, e431, e412
            self.group7() - other.group0().xyz(),
            // e235, e315, e125
            self.group8() - other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        4        0
    //    simd2        1        0        0
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        4        0
    //  no simd       14        4        0
    fn sub(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group1(),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group1(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() - other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<VersorEvenAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: VersorEvenAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]) + self.group1(),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group1(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() - other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        1        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        1        0
    //  no simd       11        1        0
    fn sub(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7() - other.group0().xyz(),
            // e235, e315, e125
            self.group8() - other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<VersorEvenAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: VersorEvenAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7() - other.group0().xyz(),
            // e235, e315, e125
            self.group8() - other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd2        1        0        0
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        4        5        0
    //  no simd       13        5        0
    fn sub(self, other: VersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group6(),
            // e423, e431, e412
            self.group7() - other.group0().xyz(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<VersorEvenOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: VersorEvenOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1() + Simd32x3::from(0.0).with_w(other[e4] * -1.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]) + self.group6(),
            // e423, e431, e412
            self.group7() - other.group0().xyz(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        1        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        1        0
    //  no simd       15        1        0
    fn sub(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group2(),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412
            self.group7() - other.group0().xyz(),
            // e235, e315, e125
            self.group8() - other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<VersorEvenOrthogonalOrigin> for MultiVector {
    fn sub_assign(&mut self, other: VersorEvenOrthogonalOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group2(),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412
            self.group7() - other.group0().xyz(),
            // e235, e315, e125
            self.group8() - other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd2        1        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        5        0
    //  no simd       17        5        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() - other.group0().xyz().with_w(other[e45]),
            // e15, e25, e35
            self.group4() - other.group2().xyz(),
            // e23, e31, e12
            self.group5() - other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] * -1.0, other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0]) + self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<VersorOdd> for MultiVector {
    fn sub_assign(&mut self, other: VersorOdd) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() - other.group0().xyz().with_w(other[e45]),
            // e15, e25, e35
            self.group4() - other.group2().xyz(),
            // e23, e31, e12
            self.group5() - other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] * -1.0, other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0]) + self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<VersorOddAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd2        1        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        5        0
    //  no simd       17        5        0
    fn sub(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35
            self.group4() - other.group0().yzw(),
            // e23, e31, e12
            self.group5() - other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0]) + self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<VersorOddAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: VersorOddAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() + Simd32x3::from(0.0).with_w(other[e45] * -1.0),
            // e15, e25, e35
            self.group4() - other.group0().yzw(),
            // e23, e31, e12
            self.group5() - other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0]) + self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<VersorOddOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd2        1        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        5        0
    //  no simd       17        5        0
    fn sub(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group3(),
            // e15, e25, e35
            self.group4() - other.group2().xyz(),
            // e23, e31, e12
            self.group5() - other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] * -1.0, 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<VersorOddOrthogonalOrigin> for MultiVector {
    fn sub_assign(&mut self, other: VersorOddOrthogonalOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group3(),
            // e15, e25, e35
            self.group4() - other.group2().xyz(),
            // e23, e31, e12
            self.group5() - other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] * -1.0, 0.0, 0.0, 0.0]) + self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
