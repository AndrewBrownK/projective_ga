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
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for AntiDipoleOnOrigin {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for AntiDipoleOnOrigin {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for AntiFlatOrigin {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for AntiFlatOrigin {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for AntiFlatOrigin {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for AntiFlatPoint {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for AntiFlatPoint {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for AntiFlectorOnOrigin {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for AntiFlectorOnOrigin {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for AntiFlectorOnOrigin {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for AntiLineOnOrigin {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for AntiLineOnOrigin {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for AntiLineOnOrigin {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for AntiMotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for AntiMotorOnOrigin {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for AntiMotorOnOrigin {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for AntiPlane {
    type Output = AntiPlane;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for AntiPlane {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for AntiPlane {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for AntiPlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for AntiPlaneOnOrigin {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for AntiPlaneOnOrigin {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for AntiScalar {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for AntiScalar {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for AntiSphereOnOrigin {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for AntiSphereOnOrigin {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for DipoleOnOrigin {
    type Output = DipoleOnOrigin;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for DipoleOnOrigin {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for DipoleOnOrigin {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for FlatOrigin {
    type Output = FlatOrigin;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for FlatOrigin {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for FlatOrigin {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for FlatPoint {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for FlatPoint {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for FlatPointAtInfinity {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for FlatPointAtInfinity {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for FlectorAtInfinity {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for FlectorAtInfinity {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for FlectorOnOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for FlectorOnOrigin {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for FlectorOnOrigin {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for Horizon {
    type Output = Horizon;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for Horizon {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for Horizon {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for Infinity {
    type Output = Infinity;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for Infinity {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for Infinity {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for LineAtInfinity {
    type Output = LineAtInfinity;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for LineAtInfinity {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for LineAtInfinity {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for LineOnOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for LineOnOrigin {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for LineOnOrigin {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for MotorAtInfinity {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for MotorAtInfinity {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for MotorOnOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for MotorOnOrigin {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for MotorOnOrigin {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for NullCircleAtOrigin {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for NullCircleAtOrigin {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for NullDipoleAtOrigin {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for NullDipoleAtOrigin {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for NullDipoleInversionAtOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for NullDipoleInversionAtOrigin {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for NullDipoleInversionAtOrigin {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for NullSphereAtOrigin {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for NullSphereAtOrigin {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for NullVersorEvenAtOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for NullVersorEvenAtOrigin {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for NullVersorEvenAtOrigin {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for Origin {
    type Output = Origin;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for Origin {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for Origin {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for Plane {
    type Output = Plane;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for Plane {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for Plane {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for PlaneOnOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for PlaneOnOrigin {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for PlaneOnOrigin {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for RoundPoint {
    type Output = RoundPoint;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for RoundPoint {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for RoundPoint {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for RoundPointAtOrigin {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for RoundPointAtOrigin {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for Scalar {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for Sphere {
    type Output = Sphere;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for Sphere {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for Sphere {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for SphereAtOrigin {
    type Output = SphereAtOrigin;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for SphereAtOrigin {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for SphereAtOrigin {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for SphereOnOrigin {
    type Output = SphereOnOrigin;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for SphereOnOrigin {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for SphereOnOrigin {
    fn constraint_valid(self) -> Self {
        self
    }
}
