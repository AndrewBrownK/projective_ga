// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 25
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       2       0
//  Average:         0       2       0
//  Maximum:         0       8       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       3       0
//  Average:         0       4       0
//  Maximum:         0      17       0
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiCircleRotor {
    type Output = CircleRotor;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiCircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn right_anti_dual(self) -> Self::Output {
        CircleRotor::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiDipoleInversion {
    type Output = DipoleInversion;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e321] * -1.0),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(self[e4] * -1.0),
            // e4235, e4315, e4125, e3215
            self.group3().xyz().with_w(self[e5] * -1.0),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiDualNum {
    type Output = DualNum;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiDualNum {
    type Output = DualNum;
    fn right_anti_dual(self) -> Self::Output {
        DualNum::from_groups(/* e5, e12345 */ self.group0())
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiFlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiFlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0().xyz().with_w(self[e321] * -1.0))
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiFlector {
    type Output = Flector;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiFlector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            self.group0().xyz().with_w(self[e321] * -1.0),
            // e4235, e4315, e4125, e3215
            self.group1().xyz().with_w(self[e5] * -1.0),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiLine {
    type Output = Line;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiLine {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn right_anti_dual(self) -> Self::Output {
        Line::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiMotor {
    type Output = Motor;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiMotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiPlane {
    type Output = Plane;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiPlane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0().xyz().with_w(self[e5] * -1.0))
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiScalar {
    type Output = Scalar;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiScalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e12345] * -1.0)
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for Circle {
    type Output = Dipole;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Circle {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e321] * -1.0),
            // e15, e25, e35
            self.group2(),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for CircleRotor {
    type Output = AntiCircleRotor;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for CircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e321] * -1.0),
            // e15, e25, e35, scalar
            self.group2().xyz().with_w(self[e12345] * -1.0),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for Dipole {
    type Output = Circle;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Dipole {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn right_anti_dual(self) -> Self::Output {
        Circle::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            self.group2() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for DipoleInversion {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn right_anti_dual(self) -> Self::Output {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            self.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for DualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for DualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn right_anti_dual(self) -> Self::Output {
        AntiDualNum::from_groups(/* e3215, scalar */ self.group0() * Simd32x2::from(-1.0))
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for FlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for FlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for Flector {
    type Output = AntiFlector;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Flector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for Line {
    type Output = AntiLine;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Line {
    type Output = AntiLine;
    fn right_anti_dual(self) -> Self::Output {
        AntiLine::from_groups(/* e23, e31, e12 */ self.group0(), /* e15, e25, e35 */ self.group1())
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for Motor {
    type Output = AntiMotor;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0().xyz().with_w(self[e12345] * -1.0),
            // e15, e25, e35, e3215
            self.group1().xyz().with_w(self[e5] * -1.0),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl std::ops::DivAssign<RightAntiDualPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: RightAntiDualPrefixOrPostfix) {
        *self = self.right_anti_dual()
    }
}
impl RightAntiDual for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       17        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            (self.group9().xyz() * Simd32x3::from(-1.0)).with_w(self[e1234]),
            // e5
            self[e3215],
            // e15, e25, e35, e45
            self.group8().with_w(self[e321] * -1.0),
            // e41, e42, e43
            self.group7(),
            // e23, e31, e12
            self.group6().xyz(),
            // e415, e425, e435, e321
            (self.group5() * Simd32x3::from(-1.0)).with_w(self[e45]),
            // e423, e431, e412
            self.group4() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group3().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group1().xyz().with_w(self[e5] * -1.0),
            // e1234
            self[e4] * -1.0,
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for Plane {
    type Output = AntiPlane;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Plane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for RoundPoint {
    type Output = Sphere;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for RoundPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0().xyz().with_w(self[e5] * -1.0), /* e1234 */ self[e4] * -1.0)
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for Scalar {
    type Output = AntiScalar;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Scalar {
    type Output = AntiScalar;
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[scalar])
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for Sphere {
    type Output = RoundPoint;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Sphere {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e1234]), /* e5 */ self[e3215])
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for VersorEven {
    type Output = VersorOdd;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().xyz().with_w(self[e12345] * -1.0),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e321] * -1.0),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(self[e4] * -1.0),
            // e4235, e4315, e4125, e3215
            self.group3().xyz().with_w(self[e5] * -1.0),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for VersorOdd {
    type Output = VersorEven;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       14        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            (self.group2().xyz() * Simd32x3::from(-1.0)).with_w(self[e3215]),
            // e1, e2, e3, e4
            (self.group3().xyz() * Simd32x3::from(-1.0)).with_w(self[e1234]),
        )
    }
}
