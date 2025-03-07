// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 36
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
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for AntiDipoleOnOrigin {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for AntiDipoleOnOrigin {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for AntiFlatOrigin {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for AntiFlatOrigin {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for AntiFlatOrigin {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for AntiFlatPoint {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for AntiFlatPoint {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for AntiFlectorOnOrigin {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for AntiFlectorOnOrigin {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for AntiFlectorOnOrigin {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for AntiLineOnOrigin {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for AntiLineOnOrigin {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for AntiLineOnOrigin {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for AntiMotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for AntiMotorOnOrigin {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for AntiMotorOnOrigin {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for AntiPlane {
    type Output = AntiPlane;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for AntiPlane {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for AntiPlane {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for AntiPlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for AntiPlaneOnOrigin {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for AntiPlaneOnOrigin {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for AntiScalar {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for AntiScalar {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for AntiSphereOnOrigin {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for AntiSphereOnOrigin {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for DipoleOnOrigin {
    type Output = DipoleOnOrigin;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for DipoleOnOrigin {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for DipoleOnOrigin {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for FlatOrigin {
    type Output = FlatOrigin;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for FlatOrigin {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for FlatOrigin {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for FlatPoint {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for FlatPoint {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for FlatPointAtInfinity {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for FlatPointAtInfinity {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for FlectorAtInfinity {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for FlectorAtInfinity {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for FlectorOnOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for FlectorOnOrigin {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for FlectorOnOrigin {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for Horizon {
    type Output = Horizon;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for Horizon {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for Horizon {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for Infinity {
    type Output = Infinity;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for Infinity {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for Infinity {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for LineAtInfinity {
    type Output = LineAtInfinity;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for LineAtInfinity {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for LineAtInfinity {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for LineOnOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for LineOnOrigin {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for LineOnOrigin {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for MotorAtInfinity {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for MotorAtInfinity {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for MotorOnOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for MotorOnOrigin {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for MotorOnOrigin {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for NullCircleAtOrigin {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for NullCircleAtOrigin {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for NullDipoleAtOrigin {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for NullDipoleAtOrigin {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for NullDipoleInversionAtOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for NullDipoleInversionAtOrigin {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for NullDipoleInversionAtOrigin {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for NullSphereAtOrigin {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for NullSphereAtOrigin {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for NullVersorEvenAtOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for NullVersorEvenAtOrigin {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for NullVersorEvenAtOrigin {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for Origin {
    type Output = Origin;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for Origin {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for Origin {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for Plane {
    type Output = Plane;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for Plane {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for Plane {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for PlaneOnOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for PlaneOnOrigin {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for PlaneOnOrigin {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for RoundPoint {
    type Output = RoundPoint;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for RoundPoint {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for RoundPoint {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for RoundPointAtOrigin {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for RoundPointAtOrigin {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for Scalar {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for Sphere {
    type Output = Sphere;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for Sphere {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for Sphere {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for SphereAtOrigin {
    type Output = SphereAtOrigin;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for SphereAtOrigin {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for SphereAtOrigin {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for SphereOnOrigin {
    type Output = SphereOnOrigin;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for SphereOnOrigin {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for SphereOnOrigin {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
