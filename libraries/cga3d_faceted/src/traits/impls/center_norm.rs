// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 25
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         3       0       0
//   Median:         6       0       0
//  Average:         5       0       0
//  Maximum:        15       0       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         3       0       0
//   Median:         6       0       0
//  Average:         5       0       0
//  Maximum:        15       0       0
impl std::ops::Div<CenterNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar] - self[e45] * self[e45],
        );
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar] - self[e45] * self[e45],
        );
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for AntiDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3] - self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435],
        );
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3] - self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435],
        );
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for AntiMysteryCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for AntiMysteryCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar] - self[e45] * self[e45],
        );
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for AntiMysteryDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for AntiMysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3] - self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435],
        );
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for Circle {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e321] * self[e321] - self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435]);
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for CircleAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e321] * self[e321] - self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435]);
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for CircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            self[e321] * self[e321] - self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435] - self[e12345] * self[e12345],
        );
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            self[e321] * self[e321] - self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435] - self[e12345] * self[e12345],
        );
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for Dipole {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] - self[e45] * self[e45]);
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for DipoleAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] - self[e45] * self[e45]);
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for DipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12]
                - self[e45] * self[e45]
                - self[e4235] * self[e4235]
                - self[e4315] * self[e4315]
                - self[e4125] * self[e4125],
        );
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12]
                - self[e45] * self[e45]
                - self[e4235] * self[e4235]
                - self[e4315] * self[e4315]
                - self[e4125] * self[e4125],
        );
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for MultiVector {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            self[scalar] * self[scalar]
                + self[e1] * self[e1]
                + self[e2] * self[e2]
                + self[e3] * self[e3]
                + self[e23] * self[e23]
                + self[e31] * self[e31]
                + self[e12] * self[e12]
                + self[e321] * self[e321]
                - self[e12345] * self[e12345]
                - self[e45] * self[e45]
                - self[e415] * self[e415]
                - self[e425] * self[e425]
                - self[e435] * self[e435]
                - self[e4235] * self[e4235]
                - self[e4315] * self[e4315]
                - self[e4125] * self[e4125],
        );
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for MysteryCircle {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for MysteryCircle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e321] * self[e321] - self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435]);
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for MysteryCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for MysteryCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            self[e321] * self[e321] - self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435] - self[e12345] * self[e12345],
        );
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for MysteryDipole {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for MysteryDipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] - self[e45] * self[e45]);
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for MysteryDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for MysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12]
                - self[e45] * self[e45]
                - self[e4235] * self[e4235]
                - self[e4315] * self[e4315]
                - self[e4125] * self[e4125],
        );
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for MysteryVersorEven {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for MysteryVersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3] + self[e321] * self[e321]
                - self[e12345] * self[e12345]
                - self[e415] * self[e415]
                - self[e425] * self[e425]
                - self[e435] * self[e435],
        );
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for MysteryVersorOdd {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for MysteryVersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            self[scalar] * self[scalar] + self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12]
                - self[e4235] * self[e4235]
                - self[e4315] * self[e4315]
                - self[e4125] * self[e4125]
                - self[e45] * self[e45],
        );
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for VersorEven {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3]
                - self[e12345] * self[e12345]
                - self[e415] * self[e415]
                - self[e425] * self[e425]
                - self[e435] * self[e435],
        );
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3] + self[e321] * self[e321]
                - self[e12345] * self[e12345]
                - self[e415] * self[e415]
                - self[e425] * self[e425]
                - self[e435] * self[e435],
        );
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for VersorOdd {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            self[scalar] * self[scalar] + self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12]
                - self[e45] * self[e45]
                - self[e4235] * self[e4235]
                - self[e4315] * self[e4315]
                - self[e4125] * self[e4125],
        );
    }
}
impl std::ops::Div<CenterNormPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormPrefixOrPostfix) -> Self::Output {
        self.center_norm()
    }
}
impl CenterNorm for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn center_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            self[scalar] * self[scalar] + self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12]
                - self[e45] * self[e45]
                - self[e4235] * self[e4235]
                - self[e4315] * self[e4315]
                - self[e4125] * self[e4125],
        );
    }
}
