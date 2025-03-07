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
//  Minimum:         0       0       0
//   Median:         3       4       0
//  Average:         4       7       0
//  Maximum:        23      33       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3      12       0
//  Average:         4      17       0
//  Maximum:        23      68       1
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
    //           add/sub      mul      div
    //      f32        7        6        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        7       12        0
    //  no simd        7       28        0
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
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(other_g0) * self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from(other_g0) * self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, scalar
            Simd32x4::from(other_g0) * self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
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
    //           add/sub      mul      div
    //      f32       10        8        0
    //    simd3        0        2        0
    //    simd4        0        5        0
    // Totals...
    // yes simd       10       15        0
    //  no simd       10       34        0
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
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other_g0) * self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(other_g0) * self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e4
            Simd32x4::from(other_g0) * self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            Simd32x4::from(other_g0) * self.group3(),
        );
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
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(f32::powi(self[scalar], -2)) * self.group0());
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
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(f32::powi(self[e321], -2)) * Simd32x4::from([self[e235] * -1.0, self[e315] * -1.0, self[e125] * -1.0, self[e321] * -1.0]),
        );
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
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3       12        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3];
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other_g0) * self.group0() * Simd32x4::from(-1.0),
            // e1, e2, e3, e5
            Simd32x4::from(other_g0) * self.group1(),
        );
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
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        4        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2       12        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12];
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(other_g0) * self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            Simd32x3::from(other_g0) * self.group1() * Simd32x3::from(-1.0),
        );
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
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       16        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar];
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(other_g0) * self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e3215
            Simd32x4::from(other_g0) * self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
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
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        4        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3]) * self.group0(),
        );
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
    //      add/sub      mul      div
    // f32        0        1        1
    fn inverse(self) -> Self {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ 1.0 / self[e12345] * -1.0);
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
    //           add/sub      mul      div
    //      f32        6        6        0
    //    simd3        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       26        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e321] * self[e321]
            - self[e415] * self[e415]
            - self[e425] * self[e425]
            - self[e435] * self[e435]
            - 2.0 * (self[e423] * self[e235])
            - 2.0 * (self[e431] * self[e315])
            - 2.0 * (self[e412] * self[e125]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(other_g0) * self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(other_g0) * self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(other_g0) * self.group2() * Simd32x3::from(-1.0),
        );
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
    //           add/sub      mul      div
    //      f32        7        6        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        7       12        0
    //  no simd        7       28        0
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
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(other_g0) * self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(other_g0) * self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e12345
            Simd32x4::from(other_g0) * self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
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
    //           add/sub      mul      div
    //      f32        6        6        0
    //    simd3        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       26        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 =
            2.0 * (self[e41] * self[e15]) + 2.0 * (self[e42] * self[e25]) + 2.0 * (self[e43] * self[e35]) + self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12]
                - self[e45] * self[e45];
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(other_g0) * self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from(other_g0) * self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35
            Simd32x3::from(other_g0) * self.group2() * Simd32x3::from(-1.0),
        );
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
    //           add/sub      mul      div
    //      f32       10        8        0
    //    simd3        0        2        0
    //    simd4        0        5        0
    // Totals...
    // yes simd       10       15        0
    //  no simd       10       34        0
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
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other_g0) * self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from(other_g0) * self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            Simd32x4::from(other_g0) * self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other_g0) * self.group3(),
        );
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
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(f32::powi(self[e12345], -2) * -1.0) * self.group0());
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
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        9        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(f32::powi(self[e45], -2) * -1.0) * Simd32x4::from([self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0, self[e45] * -1.0]),
        );
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
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3       12        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = -self[e45] * self[e45] - self[e4235] * self[e4235] - self[e4315] * self[e4315] - self[e4125] * self[e4125];
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other_g0) * self.group0() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other_g0) * self.group1(),
        );
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
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        4        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2       12        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = -self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435];
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(other_g0) * self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(other_g0) * self.group1() * Simd32x3::from(-1.0),
        );
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
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       16        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = -self[e415] * self[e415] - self[e425] * self[e425] - self[e435] * self[e435] - self[e12345] * self[e12345];
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(other_g0) * self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            Simd32x4::from(other_g0) * self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
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
    //           add/sub      mul      div
    //      f32       23       18        0
    //    simd2        0        1        0
    //    simd3        0        8        0
    //    simd4        0        6        0
    // Totals...
    // yes simd       23       33        0
    //  no simd       23       68        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = 2.0 * (self[e15] * self[e41])
            + 2.0 * (self[e25] * self[e42])
            + 2.0 * (self[e35] * self[e43])
            + 2.0 * (self[e3215] * self[e1234])
            + self[scalar] * self[scalar]
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
            - self[e4125] * self[e4125]
            - 2.0 * (self[e4] * self[e5])
            - 2.0 * (self[e423] * self[e235])
            - 2.0 * (self[e431] * self[e315])
            - 2.0 * (self[e412] * self[e125]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(other_g0) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other_g0) * self.group1(),
            // e5
            other_g0 * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(other_g0) * self.group3() * Simd32x4::from(-1.0),
            // e41, e42, e43
            Simd32x3::from(other_g0) * self.group4() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(other_g0) * self.group5() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(other_g0) * self.group6() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(other_g0) * self.group7() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(other_g0) * self.group8() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other_g0) * self.group9(),
            // e1234
            other_g0 * self[e1234],
        );
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
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        4        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(-self[e4235] * self[e4235] - self[e4315] * self[e4315] - self[e4125] * self[e4125]) * self.group0(),
        );
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
    //           add/sub      mul      div
    //      f32        3        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3        7        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3] - 2.0 * (self[e4] * self[e5]);
        return RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other_g0) * self.group0(), /* e5 */ other_g0 * self[e5]);
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
    //      add/sub      mul      div
    // f32        0        0        1
    fn inverse(self) -> Self {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ 1.0 / self[scalar]);
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
    //           add/sub      mul      div
    //      f32        3        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3        7        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = 2.0 * (self[e3215] * self[e1234]) - self[e4235] * self[e4235] - self[e4315] * self[e4315] - self[e4125] * self[e4125];
        return Sphere::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other_g0) * self.group0(), /* e1234 */ other_g0 * self[e1234]);
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
    //           add/sub      mul      div
    //      f32       11        8        0
    //    simd4        0        7        0
    // Totals...
    // yes simd       11       15        0
    //  no simd       11       36        0
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
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(other_g0) * self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            Simd32x4::from(other_g0) * self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            Simd32x4::from(other_g0) * self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(other_g0) * self.group3(),
        );
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
    //           add/sub      mul      div
    //      f32       11        8        0
    //    simd4        0        7        0
    // Totals...
    // yes simd       11       15        0
    //  no simd       11       36        0
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
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(other_g0) * self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, e45
            Simd32x4::from(other_g0) * self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            Simd32x4::from(other_g0) * self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other_g0) * self.group3(),
        );
    }
}
