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
//   Median:         0       2       0     N/A
//  Average:         0       2       0     N/A
//  Maximum:         0       8       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         0       3       0       0
//  Average:         0       4       0       0
//  Maximum:         0      17       0       0
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiCircleRotor {
    type Output = CircleRotor;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for AntiCircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       11        0        0
    fn right_dual(self) -> Self::Output {
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
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiDipoleInversion {
    type Output = DipoleInversion;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn right_dual(self) -> Self::Output {
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
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiDualNum {
    type Output = DualNum;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for AntiDualNum {
    type Output = DualNum;
    fn right_dual(self) -> Self::Output {
        DualNum::from_groups(/* e5, e12345 */ self.group0())
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiFlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for AntiFlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0().xyz().with_w(self[e321] * -1.0))
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiFlector {
    type Output = Flector;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for AntiFlector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            self.group0().xyz().with_w(self[e321] * -1.0),
            // e4235, e4315, e4125, e3215
            self.group1().xyz().with_w(self[e5] * -1.0),
        )
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiLine {
    type Output = Line;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for AntiLine {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn right_dual(self) -> Self::Output {
        Line::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiMotor {
    type Output = Motor;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for AntiMotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn right_dual(self) -> Self::Output {
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiPlane {
    type Output = Plane;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for AntiPlane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0().xyz().with_w(self[e5] * -1.0))
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiScalar {
    type Output = Scalar;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for AntiScalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e12345] * -1.0)
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for Circle {
    type Output = Dipole;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for Circle {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn right_dual(self) -> Self::Output {
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
impl std::ops::Div<RightDualPrefixOrPostfix> for CircleRotor {
    type Output = AntiCircleRotor;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for CircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn right_dual(self) -> Self::Output {
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
impl std::ops::Div<RightDualPrefixOrPostfix> for Dipole {
    type Output = Circle;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for Dipole {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       10        0        0
    fn right_dual(self) -> Self::Output {
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
impl std::ops::Div<RightDualPrefixOrPostfix> for DipoleInversion {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       15        0        0
    fn right_dual(self) -> Self::Output {
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
impl std::ops::Div<RightDualPrefixOrPostfix> for DualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for DualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd2        0        1        0      N/A
    // no simd        0        2        0        0
    fn right_dual(self) -> Self::Output {
        AntiDualNum::from_groups(/* e3215, scalar */ self.group0() * Simd32x2::from(-1.0))
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for FlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for FlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn right_dual(self) -> Self::Output {
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for Flector {
    type Output = AntiFlector;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for Flector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn right_dual(self) -> Self::Output {
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for Line {
    type Output = AntiLine;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for Line {
    type Output = AntiLine;
    fn right_dual(self) -> Self::Output {
        AntiLine::from_groups(/* e23, e31, e12 */ self.group0(), /* e15, e25, e35 */ self.group1())
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for Motor {
    type Output = AntiMotor;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0().xyz().with_w(self[e12345] * -1.0),
            // e15, e25, e35, e3215
            self.group1().xyz().with_w(self[e5] * -1.0),
        )
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl std::ops::DivAssign<RightDualPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: RightDualPrefixOrPostfix) {
        *self = self.right_dual()
    }
}
impl RightDual for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        4        0      N/A
    // Totals...
    // yes simd        0        8        0      N/A
    //  no simd        0       17        0        0
    fn right_dual(self) -> Self::Output {
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
impl std::ops::Div<RightDualPrefixOrPostfix> for Plane {
    type Output = AntiPlane;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for Plane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn right_dual(self) -> Self::Output {
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for RoundPoint {
    type Output = Sphere;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for RoundPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0().xyz().with_w(self[e5] * -1.0), /* e1234 */ self[e4] * -1.0)
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for Scalar {
    type Output = AntiScalar;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for Scalar {
    type Output = AntiScalar;
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[scalar])
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for Sphere {
    type Output = RoundPoint;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for Sphere {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e1234]), /* e5 */ self[e3215])
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for VersorEven {
    type Output = VersorOdd;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        4        0        0
    fn right_dual(self) -> Self::Output {
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
impl std::ops::Div<RightDualPrefixOrPostfix> for VersorOdd {
    type Output = VersorEven;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       14        0        0
    fn right_dual(self) -> Self::Output {
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
