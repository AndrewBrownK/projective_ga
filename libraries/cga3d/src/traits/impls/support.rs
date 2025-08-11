// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 24
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         0       1       0     N/A
//  Average:         0       1       0     N/A
//  Maximum:         0       4       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         0       2       0       0
//  Average:         0       2       0       0
//  Maximum:         0       8       0       0
impl std::ops::Div<SupportPrefixOrPostfix> for AntiCircleRotor {
    type Output = Sphere;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for AntiCircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (self.group2().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e1234
            self[e45],
        )
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
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            self.group1().xyz(),
            // e415, e425, e435, e321
            self.group2().xyz().with_w(0.0),
            // e235, e315, e125, e12345
            Simd32x3::from(0.0).with_w(self[e5] * -1.0),
        )
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
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x3::from(0.0).with_w(self[e3215]))
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
    fn support(self) -> Self::Output {
        Line::from_groups(/* e415, e425, e435 */ self.group0().xyz(), /* e235, e315, e125 */ Simd32x3::from(0.0))
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
    fn support(self) -> Self::Output {
        Motor::from_groups(/* e415, e425, e435, e12345 */ self.group0().xyz().with_w(0.0), /* e235, e315, e125, e5 */ Simd32x4::from(0.0))
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
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn support(self) -> Self::Output {
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ (self.group1() * Simd32x3::from(-1.0)).with_w(0.0))
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
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(self[e3215]),
            // e4235, e4315, e4125, e3215
            (self.group1().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
        )
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
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e5] * -1.0)
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
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(self[e12345] * -1.0), /* e5 */ 0.0)
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
    fn support(self) -> Self::Output {
        Circle::from_groups(
            // e423, e431, e412
            self.group1().xyz(),
            // e415, e425, e435, e321
            self.group2().with_w(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
        )
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
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group1().xyz(),
            // e415, e425, e435, e321
            self.group2().xyz().with_w(0.0),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(self[e12345] * -1.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        )
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
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(/* e4235, e4315, e4125, e3215 */ (self.group2() * Simd32x3::from(-1.0)).with_w(0.0), /* e1234 */ self[e45])
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
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            self.group3().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(self[e3215]),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(self[e45]),
            // e4235, e4315, e4125, e3215
            (self.group2().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
        )
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
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(self[e5] * -1.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e12345] * -1.0),
        )
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
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e1234
            self[e45],
        )
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
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            self.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(self[e3215]),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(self[e45]),
            // e4235, e4315, e4125, e3215
            (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
        )
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
        Circle::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1().with_w(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
        )
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
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(self[e5] * -1.0),
            // e415, e425, e435, e321
            self.group1().xyz().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e12345] * -1.0),
        )
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
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        8        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e5] * -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e12345] * -1.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(self[e3215]),
            // e41, e42, e43
            self.group9().xyz() * Simd32x3::from(-1.0),
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
        )
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
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            self.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(self[e3215]),
            // e15, e25, e35
            Simd32x3::from(0.0),
        )
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
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e5] * -1.0)
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
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            self.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(self[e3215]),
            // e15, e25, e35
            Simd32x3::from(0.0),
        )
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
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group1().xyz().with_w(0.0),
            // e415, e425, e435, e321
            self.group2().xyz().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e12345] * -1.0),
        )
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
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            self.group3().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(self[e3215]),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(self[e45]),
            // e4235, e4315, e4125, e3215
            (self.group2().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
        )
    }
}
