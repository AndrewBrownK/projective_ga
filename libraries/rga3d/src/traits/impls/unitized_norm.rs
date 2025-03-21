// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 5
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         0       0       0     N/A
//  Average:         0       0       0     N/A
//  Maximum:         0       0       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         0       0       0       0
//  Average:         0       0       0       0
//  Maximum:         0       0       0       0
impl std::ops::Div<UnitizedNormPrefixOrPostfix> for Flector {
    type Output = f32;
    fn div(self, _rhs: UnitizedNormPrefixOrPostfix) -> Self::Output {
        self.unitized_norm()
    }
}
impl UnitizedNorm for Flector {
    fn unitized_norm(self) -> f32 {
        0.0
    }
}
impl std::ops::Div<UnitizedNormPrefixOrPostfix> for Line {
    type Output = f32;
    fn div(self, _rhs: UnitizedNormPrefixOrPostfix) -> Self::Output {
        self.unitized_norm()
    }
}
impl UnitizedNorm for Line {
    fn unitized_norm(self) -> f32 {
        0.0
    }
}
impl std::ops::Div<UnitizedNormPrefixOrPostfix> for Motor {
    type Output = f32;
    fn div(self, _rhs: UnitizedNormPrefixOrPostfix) -> Self::Output {
        self.unitized_norm()
    }
}
impl UnitizedNorm for Motor {
    fn unitized_norm(self) -> f32 {
        0.0
    }
}
impl std::ops::Div<UnitizedNormPrefixOrPostfix> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: UnitizedNormPrefixOrPostfix) -> Self::Output {
        self.unitized_norm()
    }
}
impl UnitizedNorm for MultiVector {
    fn unitized_norm(self) -> f32 {
        0.0
    }
}
impl std::ops::Div<UnitizedNormPrefixOrPostfix> for Point {
    type Output = f32;
    fn div(self, _rhs: UnitizedNormPrefixOrPostfix) -> Self::Output {
        self.unitized_norm()
    }
}
impl UnitizedNorm for Point {
    fn unitized_norm(self) -> f32 {
        0.0
    }
}
