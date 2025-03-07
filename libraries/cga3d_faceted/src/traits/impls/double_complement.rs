// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 95
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
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = AntiCircleOnOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for AntiCircleOnOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiCircleOnOrigin {
    fn double_complement(self) -> Self {
        self
    }
}
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
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiCircleRotorAligningOrigin {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiCircleRotorAligningOriginAtInfinity {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = AntiCircleRotorAtInfinity;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiCircleRotorAtInfinity {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiCircleRotorOnOrigin {
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
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiDipoleInversionAtInfinity {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiDipoleInversionOnOrigin {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiDipoleInversionOrthogonalOrigin {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for AntiDipoleOnOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiDipoleOnOrigin {
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
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for AntiFlatOrigin {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for AntiFlatOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiFlatOrigin {
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
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for AntiFlectorOnOrigin {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for AntiFlectorOnOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiFlectorOnOrigin {
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
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for AntiLineOnOrigin {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for AntiLineOnOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiLineOnOrigin {
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
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for AntiMotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for AntiMotorOnOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiMotorOnOrigin {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for AntiMysteryCircleRotor {
    type Output = AntiMysteryCircleRotor;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for AntiMysteryCircleRotor {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiMysteryCircleRotor {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for AntiMysteryDipoleInversion {
    type Output = AntiMysteryDipoleInversion;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for AntiMysteryDipoleInversion {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiMysteryDipoleInversion {
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
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for AntiPlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for AntiPlaneOnOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiPlaneOnOrigin {
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
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for AntiSphereOnOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiSphereOnOrigin {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiVersorEvenOnOrigin {
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
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for CircleAligningOrigin {
    type Output = CircleAligningOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for CircleAligningOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for CircleAligningOrigin {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for CircleAtInfinity {
    type Output = CircleAtInfinity;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for CircleAtInfinity {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for CircleAtInfinity {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for CircleAtOrigin {
    type Output = CircleAtOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for CircleAtOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for CircleAtOrigin {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for CircleOnOrigin {
    type Output = CircleOnOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for CircleOnOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for CircleOnOrigin {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = CircleOrthogonalOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for CircleOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for CircleOrthogonalOrigin {
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
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for CircleRotorAligningOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for CircleRotorAligningOrigin {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    type Output = CircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for CircleRotorAligningOriginAtInfinity {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = CircleRotorAtInfinity;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for CircleRotorAtInfinity {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for CircleRotorAtInfinity {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for CircleRotorOnOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for CircleRotorOnOrigin {
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
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = DipoleAligningOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for DipoleAligningOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for DipoleAligningOrigin {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for DipoleAtInfinity {
    type Output = DipoleAtInfinity;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for DipoleAtInfinity {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for DipoleAtInfinity {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for DipoleAtOrigin {
    type Output = DipoleAtOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for DipoleAtOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for DipoleAtOrigin {
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
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = DipoleInversionAligningOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for DipoleInversionAligningOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for DipoleInversionAligningOrigin {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for DipoleInversionAtInfinity {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for DipoleInversionAtInfinity {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for DipoleInversionAtOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for DipoleInversionAtOrigin {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = DipoleInversionOnOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for DipoleInversionOnOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for DipoleInversionOnOrigin {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for DipoleInversionOrthogonalOrigin {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for DipoleOnOrigin {
    type Output = DipoleOnOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for DipoleOnOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for DipoleOnOrigin {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = DipoleOrthogonalOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for DipoleOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for DipoleOrthogonalOrigin {
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
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for FlatOrigin {
    type Output = FlatOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for FlatOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for FlatOrigin {
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
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for FlatPointAtInfinity {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for FlatPointAtInfinity {
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
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for FlectorAtInfinity {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for FlectorAtInfinity {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for FlectorOnOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for FlectorOnOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for FlectorOnOrigin {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for Horizon {
    type Output = Horizon;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for Horizon {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Horizon {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for Infinity {
    type Output = Infinity;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for Infinity {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Infinity {
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
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for LineAtInfinity {
    type Output = LineAtInfinity;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for LineAtInfinity {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for LineAtInfinity {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for LineOnOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for LineOnOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for LineOnOrigin {
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
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for MotorAtInfinity {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for MotorAtInfinity {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for MotorOnOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for MotorOnOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for MotorOnOrigin {
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
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for MysteryCircle {
    type Output = MysteryCircle;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for MysteryCircle {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for MysteryCircle {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for MysteryCircleRotor {
    type Output = MysteryCircleRotor;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for MysteryCircleRotor {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for MysteryCircleRotor {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for MysteryDipole {
    type Output = MysteryDipole;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for MysteryDipole {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for MysteryDipole {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for MysteryDipoleInversion {
    type Output = MysteryDipoleInversion;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for MysteryDipoleInversion {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for MysteryDipoleInversion {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for MysteryVersorEven {
    type Output = MysteryVersorEven;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for MysteryVersorEven {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for MysteryVersorEven {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for MysteryVersorOdd {
    type Output = MysteryVersorOdd;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for MysteryVersorOdd {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for MysteryVersorOdd {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for NullCircleAtOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for NullCircleAtOrigin {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for NullDipoleAtOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for NullDipoleAtOrigin {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for NullDipoleInversionAtOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for NullDipoleInversionAtOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for NullDipoleInversionAtOrigin {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for NullSphereAtOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for NullSphereAtOrigin {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for NullVersorEvenAtOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for NullVersorEvenAtOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for NullVersorEvenAtOrigin {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for Origin {
    type Output = Origin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for Origin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Origin {
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
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for PlaneOnOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for PlaneOnOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for PlaneOnOrigin {
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
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for RoundPointAtOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for RoundPointAtOrigin {
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
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for SphereAtOrigin {
    type Output = SphereAtOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for SphereAtOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for SphereAtOrigin {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for SphereOnOrigin {
    type Output = SphereOnOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for SphereOnOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for SphereOnOrigin {
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
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for VersorEvenAligningOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for VersorEvenAligningOrigin {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for VersorEvenAtInfinity {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for VersorEvenAtInfinity {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = VersorEvenAtOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for VersorEvenAtOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for VersorEvenAtOrigin {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = VersorEvenOnOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for VersorEvenOnOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for VersorEvenOnOrigin {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for VersorEvenOrthogonalOrigin {
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
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for VersorOddAtInfinity {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for VersorOddAtInfinity {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for VersorOddOrthogonalOrigin {
    fn double_complement(self) -> Self {
        self
    }
}
