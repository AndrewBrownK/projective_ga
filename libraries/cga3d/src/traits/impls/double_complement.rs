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
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         0       0       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         0       0       0
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for AntiCircleRotor {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiCircleRotor {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for AntiDipoleInversion {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiDipoleInversion {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for AntiDualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for AntiDualNum {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiDualNum {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for AntiFlatPoint {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiFlatPoint {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for AntiFlector {
    type Output = AntiFlector;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for AntiFlector {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiFlector {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for AntiLine {
    type Output = AntiLine;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for AntiLine {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiLine {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for AntiMotor {
    type Output = AntiMotor;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for AntiMotor {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiMotor {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for AntiPlane {
    type Output = AntiPlane;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for AntiPlane {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiPlane {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for AntiScalar {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiScalar {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for Circle {
    type Output = Circle;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for Circle {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Circle {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for CircleRotor {
    type Output = CircleRotor;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for CircleRotor {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for CircleRotor {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for Dipole {
    type Output = Dipole;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for Dipole {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Dipole {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for DipoleInversion {
    type Output = DipoleInversion;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for DipoleInversion {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for DipoleInversion {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for DualNum {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for DualNum {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for FlatPoint {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for FlatPoint {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for Flector {
    type Output = Flector;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for Flector {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Flector {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for Line {
    type Output = Line;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for Line {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Line {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for Motor {
    type Output = Motor;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for Motor {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Motor {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for MultiVector {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for Plane {
    type Output = Plane;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for Plane {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Plane {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for RoundPoint {
    type Output = RoundPoint;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for RoundPoint {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for RoundPoint {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Scalar {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for Sphere {
    type Output = Sphere;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for Sphere {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Sphere {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for VersorEven {
    type Output = VersorEven;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for VersorEven {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for VersorEven {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for VersorOdd {
    type Output = VersorOdd;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for VersorOdd {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for VersorOdd {
    fn double_complement(self) -> Self {
        self
    }
}
