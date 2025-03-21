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
//   Median:         0       1       0     N/A
//  Average:         0       1       0     N/A
//  Maximum:         0       6       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         0       1       0       0
//  Average:         0       4       0       0
//  Maximum:         0      17       0       0
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiCircleRotor {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       11        0        0
    fn anti_auto_morphism(self) -> Self {
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiDipoleInversion {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiDipoleInversion {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiDualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiDualNum {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiDualNum {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd2        0        1        0      N/A
    // no simd        0        2        0        0
    fn anti_auto_morphism(self) -> Self {
        AntiDualNum::from_groups(/* e3215, scalar */ self.group0() * Simd32x2::from(-1.0))
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiFlatPoint {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiFlatPoint {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiFlector {
    type Output = AntiFlector;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiFlector {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiFlector {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiLine {
    type Output = AntiLine;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiLine {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiLine {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn anti_auto_morphism(self) -> Self {
        AntiLine::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiMotor {
    type Output = AntiMotor;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiMotor {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiMotor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn anti_auto_morphism(self) -> Self {
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35, e3215
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiPlane {
    type Output = AntiPlane;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiPlane {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiPlane {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiScalar {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiScalar {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for Circle {
    type Output = Circle;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for Circle {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for Circle {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for CircleRotor {
    type Output = CircleRotor;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for CircleRotor {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for CircleRotor {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for Dipole {
    type Output = Dipole;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for Dipole {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       10        0        0
    fn anti_auto_morphism(self) -> Self {
        Dipole::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group2() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for DipoleInversion {
    type Output = DipoleInversion;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for DipoleInversion {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       15        0        0
    fn anti_auto_morphism(self) -> Self {
        DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group3() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for DualNum {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for DualNum {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for FlatPoint {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for FlatPoint {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_auto_morphism(self) -> Self {
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for Flector {
    type Output = Flector;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for Flector {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for Flector {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn anti_auto_morphism(self) -> Self {
        Flector::from_groups(
            // e15, e25, e35, e45
            self.group0() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for Line {
    type Output = Line;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for Line {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for Line {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for Motor {
    type Output = Motor;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for Motor {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for Motor {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       17        0        0
    fn anti_auto_morphism(self) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() * Simd32x4::from(-1.0),
            // e41, e42, e43
            self.group4() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group5() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9() * Simd32x4::from(-1.0),
            // e1234
            self[e1234] * -1.0,
        )
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for Plane {
    type Output = Plane;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for Plane {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for Plane {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_auto_morphism(self) -> Self {
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for RoundPoint {
    type Output = RoundPoint;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for RoundPoint {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for RoundPoint {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_auto_morphism(self) -> Self {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * -1.0)
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for Sphere {
    type Output = Sphere;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for Sphere {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_auto_morphism(self) -> Self {
        use crate::elements::*;
        Sphere::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0() * Simd32x4::from(-1.0), /* e1234 */ self[e1234] * -1.0)
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for VersorEven {
    type Output = VersorEven;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for VersorEven {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for VersorEven {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for VersorOdd {
    type Output = VersorOdd;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for VersorOdd {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for VersorOdd {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
    fn anti_auto_morphism(self) -> Self {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group3() * Simd32x4::from(-1.0),
        )
    }
}
