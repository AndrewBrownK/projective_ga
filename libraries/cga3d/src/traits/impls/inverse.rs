// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 25
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         3       8       0     N/A
//  Average:         4      11       0     N/A
//  Maximum:        23      49       2     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         3      20       0       0
//  Average:         5      20       0       0
//  Maximum:        23      70       4       0
impl std::ops::Div<InversePrefixOrPostfix> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiCircleRotor {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       13        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        7       17        0      N/A
    //  no simd        7       28        0        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = 2.0 * (self[e41] * self[e15])
            + 2.0 * (self[e42] * self[e25])
            + 2.0 * (self[e43] * self[e35])
            + self[e23] * self[e23]
            + self[e31] * self[e31]
            + self[e12] * self[e12]
            + self[scalar] * self[scalar]
            - self[e45] * self[e45];
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(other_g0 * -1.0) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other_g0 * -1.0) * self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(other_g0) * self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiDipoleInversion {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       17        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd       10       22        0      N/A
    //  no simd       10       36        0        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3]
            - self[e415] * self[e415]
            - self[e425] * self[e425]
            - self[e435] * self[e435]
            - 2.0 * (self[e423] * self[e235])
            - 2.0 * (self[e431] * self[e315])
            - 2.0 * (self[e412] * self[e125])
            - 2.0 * (self[e4] * self[e5]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other_g0 * -1.0) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other_g0 * -1.0) * self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(other_g0) * self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            Simd32x4::from(other_g0) * self.group3(),
        )
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiDualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiDualNum {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        2        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([self[e3215] / (self[scalar] * self[scalar]), 1.0 / self[scalar]]))
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiFlatPoint {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        1        0
    //    simd3        0        3        1      N/A
    // Totals...
    // yes simd        0        4        2      N/A
    //  no simd        0       10        4        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            (self.group0().xyz() * Simd32x3::from(-1.0) / (Simd32x4::from(self[e321]).xyz() * Simd32x4::from(self[e321]).xyz())).with_w(-1.0 / self[e321]),
        )
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiFlector {
    type Output = AntiFlector;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiFlector {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       13        0        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3];
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other_g0 * -1.0) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(other_g0) * self.group1(),
        )
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiLine {
    type Output = AntiLine;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiLine {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        5        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        2       11        0        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12];
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(other_g0 * -1.0) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(other_g0 * -1.0) * self.group1(),
        )
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiMotor {
    type Output = AntiMotor;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiMotor {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd        3       20        0        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar];
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(other_g0) * self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e3215
            Simd32x4::from(other_g0) * self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiPlane {
    type Output = AntiPlane;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiPlane {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        7        0        0
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd        2       11        0      N/A
    //  no simd        8       23        0        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (Simd32x4::from([self[e2] * self[e2], self[e1] * self[e1], self[e1] * self[e1], self[e2] * self[e2]]) * self.group0())
                + (self.group0() * Simd32x2::from(self[e3] * self[e3]).with_zw(self[e2] * self[e2], self[e3] * self[e3]))
                + (Simd32x4::powi(self.group0().xyzx(), 2) * self.group0()),
        )
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiScalar {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        1        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ -1.0 / self[e12345])
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for Circle {
    type Output = Circle;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for Circle {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       13        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        6       16        0      N/A
    //  no simd        6       23        0        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e321] * self[e321]
            - self[e415] * self[e415]
            - self[e425] * self[e425]
            - self[e435] * self[e435]
            - 2.0 * (self[e423] * self[e235])
            - 2.0 * (self[e431] * self[e315])
            - 2.0 * (self[e412] * self[e125]);
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(other_g0 * -1.0) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other_g0 * -1.0) * self.group1(),
            // e235, e315, e125
            Simd32x3::from(other_g0 * -1.0) * self.group2(),
        )
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for CircleRotor {
    type Output = CircleRotor;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for CircleRotor {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       13        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        7       17        0      N/A
    //  no simd        7       28        0        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e321] * self[e321]
            - self[e415] * self[e415]
            - self[e425] * self[e425]
            - self[e435] * self[e435]
            - self[e12345] * self[e12345]
            - 2.0 * (self[e423] * self[e235])
            - 2.0 * (self[e431] * self[e315])
            - 2.0 * (self[e412] * self[e125]);
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(other_g0 * -1.0) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other_g0 * -1.0) * self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(other_g0) * self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for Dipole {
    type Output = Dipole;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for Dipole {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       13        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        6       16        0      N/A
    //  no simd        6       23        0        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 =
            2.0 * (self[e41] * self[e15]) + 2.0 * (self[e42] * self[e25]) + 2.0 * (self[e43] * self[e35]) + self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12]
                - self[e45] * self[e45];
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(other_g0 * -1.0) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other_g0 * -1.0) * self.group1(),
            // e15, e25, e35
            Simd32x3::from(other_g0 * -1.0) * self.group2(),
        )
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for DipoleInversion {
    type Output = DipoleInversion;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for DipoleInversion {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       17        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd       10       22        0      N/A
    //  no simd       10       36        0        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = 2.0 * (self[e41] * self[e15])
            + 2.0 * (self[e42] * self[e25])
            + 2.0 * (self[e43] * self[e35])
            + 2.0 * (self[e1234] * self[e3215])
            + self[e23] * self[e23]
            + self[e31] * self[e31]
            + self[e12] * self[e12]
            - self[e45] * self[e45]
            - self[e4235] * self[e4235]
            - self[e4315] * self[e4315]
            - self[e4125] * self[e4125];
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other_g0 * -1.0) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other_g0 * -1.0) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(other_g0) * self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other_g0) * self.group3(),
        )
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for DualNum {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        4        2        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([self[e5] * -1.0 / (self[e12345] * self[e12345]), -1.0 / self[e12345]]))
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for FlatPoint {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        0        1        0
    //    simd3        0        2        1      N/A
    // Totals...
    // yes simd        0        2        2      N/A
    //  no simd        0        6        4        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        FlatPoint::from_groups(
            // e15, e25, e35, e45
            (self.group0().xyz() / (Simd32x4::from(self[e45]).xyz() * Simd32x4::from(self[e45]).xyz())).with_w(1.0 / self[e45]),
        )
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for Flector {
    type Output = Flector;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for Flector {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       13        0        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = -self[e45] * self[e45] - self[e4235] * self[e4235] - self[e4315] * self[e4315] - self[e4125] * self[e4125];
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other_g0 * -1.0) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other_g0) * self.group1(),
        )
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for Line {
    type Output = Line;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for Line {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        5        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        2       11        0        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = -self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435];
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(other_g0 * -1.0) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(other_g0 * -1.0) * self.group1(),
        )
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for Motor {
    type Output = Motor;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for Motor {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd        3       20        0        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = -self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435] - self[e12345] * self[e12345];
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(other_g0) * self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            Simd32x4::from(other_g0) * self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       23       40        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        4        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd       23       49        0      N/A
    //  no simd       23       70        0        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = 2.0 * (self[e41] * self[e15])
            + 2.0 * (self[e42] * self[e25])
            + 2.0 * (self[e43] * self[e35])
            + 2.0 * (self[e3215] * self[e1234])
            + self[scalar] * self[scalar]
            + self[e23] * self[e23]
            + self[e31] * self[e31]
            + self[e12] * self[e12]
            + self[e1] * self[e1]
            + self[e2] * self[e2]
            + self[e3] * self[e3]
            + self[e321] * self[e321]
            - self[e12345] * self[e12345]
            - self[e45] * self[e45]
            - self[e415] * self[e415]
            - self[e425] * self[e425]
            - self[e435] * self[e435]
            - self[e4235] * self[e4235]
            - self[e4315] * self[e4315]
            - self[e4125] * self[e4125]
            - 2.0 * (self[e423] * self[e235])
            - 2.0 * (self[e431] * self[e315])
            - 2.0 * (self[e412] * self[e125])
            - 2.0 * (self[e4] * self[e5]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(other_g0) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other_g0) * self.group1(),
            // e5
            other_g0 * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(other_g0 * -1.0) * self.group3(),
            // e41, e42, e43
            Simd32x3::from(other_g0 * -1.0) * self.group4(),
            // e23, e31, e12
            Simd32x3::from(other_g0 * -1.0) * self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(other_g0 * -1.0) * self.group6(),
            // e423, e431, e412
            Simd32x3::from(other_g0 * -1.0) * self.group7(),
            // e235, e315, e125
            Simd32x3::from(other_g0 * -1.0) * self.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other_g0) * self.group9(),
            // e1234
            other_g0 * self[e1234],
        )
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for Plane {
    type Output = Plane;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for Plane {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        7        0        0
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd        2       11        0      N/A
    //  no simd        8       23        0        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            -(Simd32x4::from([self[e4315] * self[e4315], self[e4235] * self[e4235], self[e4235] * self[e4235], self[e4315] * self[e4315]]) * self.group0())
                - (self.group0() * Simd32x2::from(self[e4125] * self[e4125]).with_zw(self[e4315] * self[e4315], self[e4125] * self[e4125]))
                - (Simd32x4::powi(self.group0().xyzx(), 2) * self.group0()),
        )
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for RoundPoint {
    type Output = RoundPoint;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for RoundPoint {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       10        0        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3] - 2.0 * (self[e4] * self[e5]);
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other_g0) * self.group0(), /* e5 */ other_g0 * self[e5])
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        0        1        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ 1.0 / self[scalar])
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for Sphere {
    type Output = Sphere;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for Sphere {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       10        0        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = 2.0 * (self[e3215] * self[e1234]) - self[e4235] * self[e4235] - self[e4315] * self[e4315] - self[e4125] * self[e4125];
        Sphere::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other_g0) * self.group0(), /* e1234 */ other_g0 * self[e1234])
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for VersorEven {
    type Output = VersorEven;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for VersorEven {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       17        0        0
    //    simd4        0        6        0      N/A
    // Totals...
    // yes simd       11       23        0      N/A
    //  no simd       11       41        0        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3]
            - self[e12345] * self[e12345]
            - self[e415] * self[e415]
            - self[e425] * self[e425]
            - self[e435] * self[e435]
            - 2.0 * (self[e423] * self[e235])
            - 2.0 * (self[e431] * self[e315])
            - 2.0 * (self[e412] * self[e125])
            - 2.0 * (self[e5] * self[e4]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(other_g0) * self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            Simd32x4::from(other_g0 * -1.0) * self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(other_g0) * self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(other_g0) * self.group3(),
        )
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for VersorOdd {
    type Output = VersorOdd;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for VersorOdd {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       17        0        0
    //    simd4        0        6        0      N/A
    // Totals...
    // yes simd       11       23        0      N/A
    //  no simd       11       41        0        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = 2.0 * (self[e41] * self[e15])
            + 2.0 * (self[e42] * self[e25])
            + 2.0 * (self[e43] * self[e35])
            + 2.0 * (self[e1234] * self[e3215])
            + self[scalar] * self[scalar]
            + self[e23] * self[e23]
            + self[e31] * self[e31]
            + self[e12] * self[e12]
            - self[e45] * self[e45]
            - self[e4235] * self[e4235]
            - self[e4315] * self[e4315]
            - self[e4125] * self[e4125];
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(other_g0) * self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, e45
            Simd32x4::from(other_g0 * -1.0) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(other_g0) * self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other_g0) * self.group3(),
        )
    }
}
