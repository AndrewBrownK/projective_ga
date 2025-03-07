// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 24
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         0       1       0
//  Maximum:         2       5       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       4       0
//  Average:         0       5       0
//  Maximum:         8      17       0
impl std::ops::Div<SupportPrefixOrPostfix> for AntiCircleRotor {
    type Output = Sphere;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for AntiCircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0])).xyz().with_w(0.0),
            // e1234
            self[e45],
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for AntiDipoleInversion {
    type Output = CircleRotor;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for AntiDipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            (self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0])).xyz(),
            // e415, e425, e435, e321
            (self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0])).xyz().with_w(0.0),
            // e235, e315, e125, e12345
            Simd32x3::from(0.0).with_w(self[e5] * -1.0),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for AntiDualNum {
    type Output = FlatPoint;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for AntiDualNum {
    type Output = FlatPoint;
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x3::from(0.0).with_w(self[e3215]));
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for AntiFlatPoint {
    type Output = Line;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for AntiFlatPoint {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn support(self) -> Self::Output {
        return Line::from_groups(
            // e415, e425, e435
            (self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0])).xyz(),
            // e235, e315, e125
            Simd32x3::from(0.0),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for AntiFlector {
    type Output = Motor;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for AntiFlector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        3        0
    // no simd        4       12        0
    fn support(self) -> Self::Output {
        let right_anti_dual_g1 = self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x3::from(0.0).with_w(1.0).wwwx() * (self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0])).xyz().with_w(right_anti_dual_g1[0]))
                + Simd32x3::from(0.0).with_w(right_anti_dual_g1[3]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for AntiLine {
    type Output = Plane;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for AntiLine {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn support(self) -> Self::Output {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ (self.group1() * Simd32x3::from(-1.0)).with_w(0.0));
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for AntiMotor {
    type Output = Flector;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for AntiMotor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn support(self) -> Self::Output {
        let right_anti_dual_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(right_anti_dual_g1[3]),
            // e4235, e4315, e4125, e3215
            right_anti_dual_g1.xyz().with_w(0.0),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for AntiPlane {
    type Output = AntiScalar;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for AntiPlane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e5] * -1.0);
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for AntiScalar {
    type Output = RoundPoint;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for AntiScalar {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(self[e12345] * -1.0), /* e5 */ 0.0);
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for Circle {
    type Output = Circle;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl std::ops::DivAssign<SupportPrefixOrPostfix> for Circle {
    fn div_assign(&mut self, _rhs: SupportPrefixOrPostfix) {
        *self = self.support()
    }
}
impl Support for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn support(self) -> Self::Output {
        return Circle::from_groups(
            // e423, e431, e412
            (self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0])).xyz(),
            // e415, e425, e435, e321
            self.group2().with_w(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for CircleRotor {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for CircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn support(self) -> Self::Output {
        let right_anti_dual_g2 = self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0])).xyz(),
            // e415, e425, e435, e321
            right_anti_dual_g2.xyz().with_w(0.0),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(right_anti_dual_g2[3]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for Dipole {
    type Output = Sphere;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for Dipole {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return Sphere::from_groups(/* e4235, e4315, e4125, e3215 */ (self.group2() * Simd32x3::from(-1.0)).with_w(0.0), /* e1234 */ self[e45]);
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for DipoleInversion {
    type Output = DipoleInversion;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl std::ops::DivAssign<SupportPrefixOrPostfix> for DipoleInversion {
    fn div_assign(&mut self, _rhs: SupportPrefixOrPostfix) {
        *self = self.support()
    }
}
impl Support for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3 = self.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            right_anti_dual_g3.xyz(),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(right_anti_dual_g3[3]),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(self[e45]),
            // e4235, e4315, e4125, e3215
            (self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0])).xyz().with_w(0.0),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for DualNum {
    type Output = VersorEven;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for DualNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn support(self) -> Self::Output {
        let right_anti_dual_g0 = self.group0() * Simd32x2::from(-1.0);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(right_anti_dual_g0[0]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(right_anti_dual_g0[1]),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for FlatPoint {
    type Output = Sphere;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for FlatPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn support(self) -> Self::Output {
        let right_anti_dual_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Sphere::from_groups(/* e4235, e4315, e4125, e3215 */ right_anti_dual_g0.xyz().with_w(0.0), /* e1234 */ right_anti_dual_g0[3]);
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for Flector {
    type Output = DipoleInversion;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for Flector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn support(self) -> Self::Output {
        let right_anti_dual_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_anti_dual_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            right_anti_dual_g1.xyz(),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(right_anti_dual_g1[3]),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(right_anti_dual_g0[3]),
            // e4235, e4315, e4125, e3215
            right_anti_dual_g0.xyz().with_w(0.0),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for Line {
    type Output = Circle;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for Line {
    type Output = Circle;
    fn support(self) -> Self::Output {
        return Circle::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for Motor {
    type Output = VersorEven;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn support(self) -> Self::Output {
        let right_anti_dual_g0 = self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_anti_dual_g1 = self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            right_anti_dual_g0.xyz().with_w(right_anti_dual_g1[3]),
            // e415, e425, e435, e321
            right_anti_dual_g1.xyz().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(right_anti_dual_g0[3]),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl std::ops::DivAssign<SupportPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: SupportPrefixOrPostfix) {
        *self = self.support()
    }
}
impl Support for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        9        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e5] * -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self.group0().yx()[0] * -1.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(self[e3215]),
            // e41, e42, e43
            (self.group9().xyz().with_w(self[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0])).xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group8().with_w(0.0),
            // e423, e431, e412
            self.group6().xyz(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            (self.group3().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e1234
            self[e45],
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for Plane {
    type Output = Dipole;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for Plane {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn support(self) -> Self::Output {
        let right_anti_dual_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Dipole::from_groups(
            // e41, e42, e43
            right_anti_dual_g0.xyz(),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(right_anti_dual_g0[3]),
            // e15, e25, e35
            Simd32x3::from(0.0),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for RoundPoint {
    type Output = AntiScalar;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for RoundPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e5] * -1.0);
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for Sphere {
    type Output = Dipole;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for Sphere {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            (self.group0().xyz().with_w(self[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0])).xyz(),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(self[e3215]),
            // e15, e25, e35
            Simd32x3::from(0.0),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for VersorEven {
    type Output = VersorEven;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl std::ops::DivAssign<SupportPrefixOrPostfix> for VersorEven {
    fn div_assign(&mut self, _rhs: SupportPrefixOrPostfix) {
        *self = self.support()
    }
}
impl Support for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        8       17        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3 = self.group3().xyz().with_w(self[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x3::from(0.0).with_w(1.0).wwwy() * (self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0])).xyz().with_w(right_anti_dual_g3[1]))
                + Simd32x3::from(0.0).with_w(right_anti_dual_g3[0])
                + Simd32x3::from(0.0).with_w(right_anti_dual_g3[3]),
            // e415, e425, e435, e321
            (self.group2().xyz().with_w(self[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0])).xyz().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e12345] * -1.0),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for VersorOdd {
    type Output = DipoleInversion;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for VersorOdd {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2 = self.group2().xyz().with_w(self[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (self.group3().xyz().with_w(self[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0])).xyz(),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(right_anti_dual_g2[3]),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(self[e45]),
            // e4235, e4315, e4125, e3215
            right_anti_dual_g2.xyz().with_w(0.0),
        );
    }
}
